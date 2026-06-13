//! Consolidated flame screensaver effect module.
//!
//! **Taxonomy Classification**: System Role (Purpose - Application Software).

use std::time::Duration;
use crate::runner::core::{LcgRng, TerminalCell};
use crate::runner::core::screensaver::Screensaver;
use crate::runner::toolkit::sys_info::get_system_info;
use crate::runner::toolkit::sys_info::query_current_palette;

mod types;
mod physics;
mod stars;
mod rendering;
mod update;
mod sparks;
mod globs;

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;

// Re-export or use internal types
pub use types::{Spark, LogoCell, Star, VolcanicGlob};

pub struct Flame {
    pub(crate) rng: LcgRng,
    pub(crate) fire_grid: Vec<u8>,
    pub(crate) sparks: Vec<Spark>,
    pub(crate) logo_cells: Vec<LogoCell>,
    pub(crate) stars: Vec<Star>,
    pub(crate) volcanic_globs: Vec<VolcanicGlob>,
    pub(crate) time_elapsed: f32,
    pub(crate) physics_accumulator: f32,
    pub(crate) last_cols: usize,
    pub(crate) last_rows: usize,
    pub(crate) palette: [(u8, u8, u8); 36],
    pub(crate) flame_height_opt: u32,
    pub(crate) spark_count_opt: u32,

    // Live system dynamics
    pub(crate) sys_refresh_timer: f32,
    pub(crate) mem_pressure: f32,
    pub(crate) cpu_load: f32,
    pub(crate) _host_bias: f32,
    pub(super) on_battery: bool,
    pub(super) frame_time_ema: f32,
    pub(super) quality_scale: f32,
    pub(super) target_frame_time: f32,
}

impl Default for Flame {
    fn default() -> Self {
        Self::new()
    }
}

impl Flame {
    pub fn new() -> Self {
        // Pre-4.1 HKEY_CURRENT_USER registry reads (FlameHeight, SparkCount)
        // collapsed to defaults for the inline migration. Re-added in 4.2.
        let flame_height_opt: u32 = 1;
        let spark_count_opt: u32 = 1;

        // library 4.0: pull the accent + the fire heat ramp from the canonical
        // ScreenPalette. The local `get_palette(accent)` helper is a
        // fire-specific heat ramp (not accent-derived) so we still call
        // it directly, but we pass the library-routed accent through so
        // a future palette change propagates.
        let accent = query_current_palette().accent;
        let palette = physics::get_palette(accent);

        let sys = get_system_info();
        let host_bias = sys.hostname.chars().map(|c| c as u32).sum::<u32>() as f32 / 1000.0 % 1.0;
        let mem_pressure = sys.mem_used_pct / 100.0;
        let cpu_load = (sys.cpu_usage_pct / 100.0).clamp(0.0, 1.0);
        let on_battery = sys.power_status.contains("Battery");

        Self {
            rng: LcgRng::new(9999),
            fire_grid: Vec::new(),
            sparks: Vec::new(),
            logo_cells: Vec::new(),
            stars: Vec::new(),
            volcanic_globs: Vec::new(),
            time_elapsed: 0.0,
            physics_accumulator: 0.0,
            last_cols: 0,
            last_rows: 0,
            palette,
            flame_height_opt,
            spark_count_opt,
            sys_refresh_timer: 0.0,
            mem_pressure,
            cpu_load,
            _host_bias: host_bias,
            on_battery,
            frame_time_ema: 0.01666667,
            quality_scale: 1.0,
            target_frame_time: 0.01666667,
        }
    }
}

impl Screensaver for Flame {
    fn update(&mut self, dt: Duration, cols: usize, rows: usize) {
        let delta = self.update_time_and_metrics(dt);
        self.handle_resize(cols, rows);
        self.update_stars(cols, rows, delta);

        // Fixed timestep step for fire cellular automata at 32 Hz (with spiral safety limit)
        let physics_step = 0.031;
        if self.physics_accumulator > physics_step * 2.0 {
            self.physics_accumulator = physics_step * 2.0;
        }
        while self.physics_accumulator >= physics_step {
            self.physics_accumulator -= physics_step;
            physics::step_fire(self, cols, rows);
        }

        self.update_sparks(cols, rows, delta);
        self.update_volcanic_globs(cols, rows, delta);
    }

    fn draw(&self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        rendering::draw_fire(self, grid, cols, rows);
    }
}
