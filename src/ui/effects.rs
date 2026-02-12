use std::time::Duration;

use ratatui::style::Color;

/// Active visual effects.
#[derive(Debug, Clone)]
pub struct Effects {
    /// Board shake offset (x, y) in cells.
    pub shake: (f32, f32),
    /// Shake velocity for spring physics.
    shake_vel: (f32, f32),

    /// Danger flash intensity (0.0 - 1.0).
    pub danger_flash: f32,
    pub in_danger: bool,

    /// Line clear flash: (rows, progress 0.0-1.0).
    pub line_clear_flash: Option<(Vec<usize>, f32)>,

    /// Hard drop flash timer.
    pub hard_drop_flash: Duration,

    /// Lock flash timer.
    pub lock_flash: Duration,

    /// Perfect clear celebration timer.
    pub pc_timer: Duration,

    /// Action text opacity (1.0 = fully visible).
    pub action_text_opacity: f32,
}

const SPRING_CONSTANT: f32 = 0.02;
const FRICTION: f32 = 0.75;

impl Effects {
    pub fn new() -> Self {
        Self {
            shake: (0.0, 0.0),
            shake_vel: (0.0, 0.0),
            danger_flash: 0.0,
            in_danger: false,
            line_clear_flash: None,
            hard_drop_flash: Duration::ZERO,
            lock_flash: Duration::ZERO,
            pc_timer: Duration::ZERO,
            action_text_opacity: 0.0,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        let dt_f = dt.as_secs_f32();

        // Spring physics for shake
        let force_x = -SPRING_CONSTANT * self.shake.0;
        let force_y = -SPRING_CONSTANT * self.shake.1;
        self.shake_vel.0 = (self.shake_vel.0 + force_x) * FRICTION;
        self.shake_vel.1 = (self.shake_vel.1 + force_y) * FRICTION;
        self.shake.0 += self.shake_vel.0;
        self.shake.1 += self.shake_vel.1;

        // Clamp tiny values to zero
        if self.shake.0.abs() < 0.01 && self.shake_vel.0.abs() < 0.01 {
            self.shake.0 = 0.0;
            self.shake_vel.0 = 0.0;
        }
        if self.shake.1.abs() < 0.01 && self.shake_vel.1.abs() < 0.01 {
            self.shake.1 = 0.0;
            self.shake_vel.1 = 0.0;
        }

        // Danger flash
        if self.in_danger {
            self.danger_flash = (self.danger_flash + dt_f * 3.0).min(1.0);
        } else {
            self.danger_flash = (self.danger_flash - dt_f * 3.0).max(0.0);
        }

        // Line clear flash
        if let Some((_, ref mut progress)) = self.line_clear_flash {
            *progress += dt_f * 5.0;
            if *progress >= 1.0 {
                self.line_clear_flash = None;
            }
        }

        // Timers
        self.hard_drop_flash = self.hard_drop_flash.saturating_sub(dt);
        self.lock_flash = self.lock_flash.saturating_sub(dt);
        self.pc_timer = self.pc_timer.saturating_sub(dt);

        // Action text fade
        self.action_text_opacity = (self.action_text_opacity - dt_f * 0.5).max(0.0);
    }

    /// Trigger hard drop visual feedback.
    pub fn trigger_hard_drop(&mut self, _cells: u32) {
        self.hard_drop_flash = Duration::from_millis(80);
    }

    /// Trigger piece lock flash.
    pub fn trigger_lock(&mut self) {
        self.lock_flash = Duration::from_millis(50);
    }

    /// Trigger line clear animation.
    pub fn trigger_line_clear(&mut self, rows: Vec<usize>) {
        self.line_clear_flash = Some((rows, 0.0));
    }

    /// Trigger perfect clear celebration.
    pub fn trigger_pc(&mut self) {
        self.pc_timer = Duration::from_millis(2000);
    }

    /// Show action text.
    pub fn trigger_action_text(&mut self) {
        self.action_text_opacity = 1.0;
    }

    /// Set danger state.
    pub fn set_danger(&mut self, danger: bool) {
        self.in_danger = danger;
    }

    /// Get shake offset in terminal cells.
    pub fn shake_offset(&self) -> (i16, i16) {
        (self.shake.0.round() as i16, self.shake.1.round() as i16)
    }

    /// Get flash color for line clear animation.
    pub fn line_clear_color(&self, progress: f32) -> Color {
        let phase = (progress * 3.0) as u8;
        match phase % 3 {
            0 => Color::Rgb(255, 255, 255),
            1 => Color::Rgb(200, 220, 255),
            _ => Color::Rgb(150, 180, 255),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
