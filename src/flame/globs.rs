use super::{Flame, Spark, VolcanicGlob};

impl Flame {
    pub fn update_volcanic_globs(&mut self, cols: usize, rows: usize, delta: f32) {
        // Launch a new volcanic glob randomly
        let launch_chance = 0.015 * (1.0 + self.cpu_load);
        if self.volcanic_globs.len() < 3 && self.rng.next_bool(launch_chance) {
            let launch_left = self.rng.next_bool(0.5);
            let start_x = if launch_left {
                self.rng.next_range(2.0, (cols as f32 * 0.25).max(4.0))
            } else {
                self.rng.next_range((cols as f32 * 0.75).min(cols as f32 - 4.0), cols as f32 - 2.0)
            };
            let start_y = rows as f32 - 1.0;

            let speed_scale = (cols as f32 / 80.0).clamp(0.5, 2.5);
            let vx = if launch_left {
                self.rng.next_range(14.0, 26.0) * speed_scale
            } else {
                -self.rng.next_range(14.0, 26.0) * speed_scale
            };

            let gravity = 12.0f32;
            let target_height = rows as f32 * self.rng.next_range(0.5, 0.75);
            let vy = -(2.0 * gravity * target_height).sqrt();

            self.volcanic_globs.push(VolcanicGlob {
                x: start_x,
                y: start_y,
                vx,
                vy,
                life: 4.5,
            });
        }

        // Update volcanic globs
        let mut exploded_globs = Vec::new();
        let gravity = 12.0f32;

        for (idx, glob) in self.volcanic_globs.iter_mut().enumerate() {
            glob.vy += gravity * delta;
            glob.x += glob.vx * delta;
            glob.y += glob.vy * delta;
            glob.life -= delta;

            if self.rng.next_bool(0.35) {
                self.sparks.push(Spark {
                    x: glob.x,
                    y: glob.y,
                    vx: -glob.vx * 0.15 + self.rng.next_range(-0.5, 0.5),
                    vy: -glob.vy * 0.15 + self.rng.next_range(-0.5, 0.5),
                    life: 0.8,
                    max_life: 0.8,
                });
            }

            let mut hit = false;
            for cell in &mut self.logo_cells {
                let dx = glob.x - cell.x as f32;
                let dy = (glob.y - cell.y as f32) * 2.0;
                let dist = (dx*dx + dy*dy).sqrt();
                if dist < 1.6 {
                    hit = true;
                    cell.temp = 3.0; 
                }
            }

            if hit {
                exploded_globs.push((idx, glob.x, glob.y));
            }
        }

        // Handle glob explosions
        for (idx, x, y) in exploded_globs.into_iter().rev() {
            self.volcanic_globs.remove(idx);

            for _ in 0..25 {
                let angle = self.rng.next_range(0.0, std::f32::consts::TAU);
                let speed = self.rng.next_range(7.0, 16.0);
                self.sparks.push(Spark {
                    x,
                    y,
                    vx: angle.cos() * speed,
                    vy: angle.sin() * speed * 0.5 - 2.0,
                    life: self.rng.next_range(0.6, 1.6),
                    max_life: 1.6,
                });
            }

            let ex = x.round() as i32;
            let ey = y.round() as i32;
            let r_int = 4;
            for dy in -r_int..=r_int {
                for dx in -r_int..=r_int {
                    let px = ex + dx;
                    let py = ey + dy;
                    if px >= 0 && px < cols as i32 && py >= 0 && py < rows as i32
                        && (dx*dx + dy*dy) as f32 <= 16.0 {
                        let grid_idx = py as usize * cols + px as usize;
                        self.fire_grid[grid_idx] = 35;
                    }
                }
            }
        }
        self.volcanic_globs.retain(|g| g.life > 0.0 && g.x >= 0.0 && g.x < cols as f32 && g.y < rows as f32);
    }
}
