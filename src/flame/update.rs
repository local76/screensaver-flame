use std::time::Duration;
use crate::runner::toolkit::sys_info::get_system_info;
use crate::runner::toolkit::sys_info::query_current_palette;
use crate::runner::core::logo_block::render_logo_block;
use super::{Flame, LogoCell, Star};
use super::physics;

impl Flame {
    pub fn update_time_and_metrics(&mut self, dt: Duration) -> f32 {
        let dt_secs = dt.as_secs_f32();

        // Auto-detect high refresh rates during the startup phase
        if self.time_elapsed < 2.0 && dt_secs > 0.001 {
            if dt_secs < self.target_frame_time - 0.001 {
                self.target_frame_time = dt_secs;
            }
        }

        // Exponential moving average for frame time (alpha = 0.1)
        self.frame_time_ema = self.frame_time_ema * 0.9 + dt_secs.min(0.2) * 0.1;

        let speed_mult = if self.on_battery { 0.65 } else { 1.0 };
        let delta = dt_secs * speed_mult;
        self.time_elapsed += delta;
        self.physics_accumulator += delta;

        // Adjust quality_scale based on frame time performance vs target
        if self.time_elapsed > 1.5 {
            if self.frame_time_ema > self.target_frame_time * 1.15 {
                self.quality_scale = (self.quality_scale - 0.15 * delta).max(0.20);
            } else if self.frame_time_ema < self.target_frame_time * 1.05 {
                self.quality_scale = (self.quality_scale + 0.04 * delta).min(1.0);
            }
        }

        // Live system refresh ~every sec
        if self.sys_refresh_timer >= 0.0 {
            self.sys_refresh_timer += delta;
            if self.sys_refresh_timer >= 1.0 {
                let sys = get_system_info();
                self.mem_pressure = sys.mem_used_pct / 100.0;
                self.cpu_load = (sys.cpu_usage_pct / 100.0).clamp(0.0, 1.0);
                self.on_battery = sys.power_status.contains("Battery");
                self.sys_refresh_timer = 0.0;
            }
        }

        delta
    }

    pub fn handle_resize(&mut self, cols: usize, rows: usize) {
        if cols != self.last_cols || rows != self.last_rows {
            self.fire_grid = vec![0; cols * rows];
            self.sparks.clear();
            self.logo_cells.clear();
            self.volcanic_globs.clear();
            self.stars.clear();

            let logo_text = get_system_info().logo_text;
            let lines = render_logo_block(&logo_text, None);
            let logo_h = lines.len();
            let logo_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
            let logo_x = cols.saturating_sub(logo_w) / 2;
            let logo_y = rows.saturating_sub(logo_h) / 2;

            for (r_offset, line) in lines.iter().enumerate().take(logo_h) {
                for (c_offset, ch) in line.chars().enumerate() {
                    if ch != ' ' {
                        self.logo_cells.push(LogoCell {
                            x: logo_x + c_offset,
                            y: logo_y + r_offset,
                            ch,
                            temp: 0.0,
                        });
                    }
                }
            }

            self.last_cols = cols;
            self.last_rows = rows;

            let accent = query_current_palette().accent;
            self.palette = physics::get_palette(accent);
        }
    }

    pub fn update_stars(&mut self, cols: usize, rows: usize, delta: f32) {
        // Dynamically adjust star population to match target capacity
        let target_stars = (((cols * rows / 20).clamp(10, 80)) as f32 * self.quality_scale * (if self.on_battery { 0.55 } else { 1.0 })) as usize;
        if self.stars.len() > target_stars {
            self.stars.truncate(target_stars);
        } else if self.stars.len() < target_stars && target_stars > 0 {
            while self.stars.len() < target_stars {
                self.stars.push(Star {
                    x: self.rng.next_f32(),
                    y: self.rng.next_f32(),
                    phase: self.rng.next_f32() * std::f32::consts::TAU,
                    ch: if self.stars.len() % 7 == 0 { '✦' } else if self.stars.len() % 3 == 0 { '•' } else { '.' },
                    excitation: 0.0,
                    excited_color: (255, 255, 255),
                });
            }
        }

        // Decay star excitations
        for star in &mut self.stars {
            if star.excitation > 0.0 {
                star.excitation -= delta * 1.8;
                if star.excitation < 0.0 { star.excitation = 0.0; }
            }
        }
    }
}
