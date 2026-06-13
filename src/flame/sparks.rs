use super::{Flame, Spark};

impl Flame {
    pub fn update_sparks(&mut self, cols: usize, rows: usize, delta: f32) {
        // 3. Update logo temperature and spawn sparks
        for cell in &mut self.logo_cells {
            if cell.x >= cols || cell.y >= rows {
                continue;
            }
            let mut column_heat = 0.0;
            let check_depth = 12;
            for dy in 1..=check_depth {
                let check_y = cell.y + dy;
                if check_y < rows {
                    column_heat += self.fire_grid[check_y * cols + cell.x] as f32;
                }
            }
            let average_heat = column_heat / (check_depth as f32 * 35.0);
            cell.temp = cell.temp * 0.86 + average_heat * 0.14;

            let mut spark_logo_prob = match self.spark_count_opt {
                0 => 0.0135,
                2 => 0.1125,
                _ => 0.045,
            };
            spark_logo_prob *= self.quality_scale * (if self.on_battery { 0.55 } else { 1.0 });

            if cell.temp > 0.15 && self.rng.next_bool(spark_logo_prob) {
                self.sparks.push(Spark {
                    x: cell.x as f32,
                    y: cell.y as f32,
                    vx: self.rng.next_range(-1.8, 1.8),
                    vy: -self.rng.next_range(4.5, 9.5),
                    life: self.rng.next_range(0.8, 2.0),
                    max_life: 2.0,
                });
            }
        }

        // Spawn sparks from top of the fire grid columns
        let mut spark_top_prob = match self.spark_count_opt {
            0 => 0.072,
            2 => 0.60,
            _ => 0.24,
        };
        spark_top_prob *= self.quality_scale * (if self.on_battery { 0.55 } else { 1.0 });

        if self.rng.next_bool(spark_top_prob) {
            let x = self.rng.next_usize(cols);
            for y in (rows / 2..rows - 2).rev() {
                let val = self.fire_grid[y * cols + x];
                if (6..=24).contains(&val) {
                    self.sparks.push(Spark {
                        x: x as f32,
                        y: y as f32,
                        vx: self.rng.next_range(-2.0, 2.0),
                        vy: -self.rng.next_range(5.5, 12.0),
                        life: self.rng.next_range(0.9, 2.3),
                        max_life: 2.3,
                    });
                    break;
                }
            }
        }

        // 4. Update spark velocities
        for spark in &mut self.sparks {
            let wind = (self.time_elapsed * 2.3 + spark.y * 0.08).sin() * 4.5;
            spark.vx += wind * delta;
            spark.vx = spark.vx.clamp(-8.0, 8.0);

            spark.x += spark.vx * delta;
            spark.y += spark.vy * delta;
            spark.life -= delta;
        }

        self.sparks.retain(|s| s.life > 0.0 && s.x >= 0.0 && s.x < cols as f32 && s.y >= 0.0 && s.y < rows as f32);

        // Excite background stars near sparks
        for spark in &self.sparks {
            for star in &mut self.stars {
                let sx = star.x * cols as f32;
                let sy = star.y * rows as f32;
                let dx = spark.x - sx;
                let dy = (spark.y - sy) * 2.0;
                let dist_sq = dx*dx + dy*dy;
                if dist_sq < 9.0 {
                    let dist = dist_sq.sqrt();
                    let force = (1.0 - dist / 3.0) * 1.5;
                    if force > star.excitation {
                        star.excitation = force;
                        let t = self.rng.next_f32();
                        star.excited_color = (255, (160.0 + t * 90.0) as u8, 0);
                    }
                }
            }
        }
    }
}
