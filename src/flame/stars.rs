use crate::runner::core::TerminalCell;
use super::Flame;

pub fn draw_stars_and_flares(effect: &Flame, grid: &mut [TerminalCell], cols: usize, rows: usize) {
    // Find top candidates for lens flares (only highly excited stars, max 4)
    let mut flare_candidates: Vec<(usize, f32)> = effect.stars.iter()
        .enumerate()
        .filter(|(_, star)| star.excitation > 0.8)
        .map(|(idx, star)| (idx, star.excitation))
        .collect();
    flare_candidates.sort_by(|a, b| b.1.total_cmp(&a.1));
    let allowed_flares: Vec<usize> = flare_candidates.iter()
        .take(4)
        .map(|&(idx, _)| idx)
        .collect();

    // 1. Draw background stars & lens flares (illuminated and excited by sparks)
    for (i, star) in effect.stars.iter().enumerate() {
        let sx = (star.x * cols as f32) as usize;
        let sy = (star.y * rows as f32) as usize;
        if sx < cols && sy < rows {
            // Only draw if there is no fire at this location
            if effect.fire_grid[sy * cols + sx] == 0 {
                // Base twinkle brightness
                let sparkle_base = ((effect.time_elapsed * 2.0 + star.phase).sin() + 1.0) * 0.5;
                let sparkle = (sparkle_base + star.excitation).min(2.0);
                let base_brightness = (sparkle_base * 120.0 + 40.0) as u8;

                let mut r = base_brightness;
                let mut g = base_brightness;
                let mut b = base_brightness.saturating_add(25);

                if star.excitation > 0.05 {
                    let blend = (star.excitation * 0.7).min(1.0);
                    r = (r as f32 * (1.0 - blend) + star.excited_color.0 as f32 * blend).min(255.0) as u8;
                    g = (g as f32 * (1.0 - blend) + star.excited_color.1 as f32 * blend).min(255.0) as u8;
                    b = (b as f32 * (1.0 - blend) + star.excited_color.2 as f32 * blend).min(255.0) as u8;
                }

                let final_brightness = sparkle * 0.4;

                let ch = if final_brightness > 0.8 {
                    '✹'
                } else if final_brightness > 0.5 {
                    '✦'
                } else {
                    star.ch
                };

                grid[sy * cols + sx] = TerminalCell {
                    ch,
                    fg: (r, g, b),
                    bg: (0, 0, 0),
                    bold: final_brightness > 0.6 || star.excitation > 0.3,
                };

                // Draw lens flares and starbursts on highly excited stars
                let is_excited = allowed_flares.contains(&i);
                if is_excited {
                    let flare_intensity = ((star.excitation - 0.8) / 0.7 + 0.5).min(1.5);
                    let flare_color = star.excited_color;

                    // Draw horizontal flare (cinematic anamorphic streak, longer)
                    let h_len = 12;
                    for dx in 1..h_len {
                        let alpha = (120.0f32 * flare_intensity).max(30.0f32) as u8;
                        let fade = alpha.saturating_sub((dx * (110 / h_len)) as u8);
                        if fade > 10 {
                            if sx + dx < cols {
                                let cell = &mut grid[sy * cols + (sx + dx)];
                                if effect.fire_grid[sy * cols + (sx + dx)] == 0 && (cell.ch == ' ' || cell.ch == '─') {
                                    cell.ch = '─';
                                    let fg_r = fade.saturating_add((flare_color.0 as f32 * 0.8) as u8);
                                    let fg_g = ((fade as f32 * 0.75) as u8).saturating_add((flare_color.1 as f32 * 0.8) as u8);
                                    let fg_b = (fade.saturating_add(45)).saturating_add((flare_color.2 as f32 * 0.8) as u8);
                                    cell.fg = (fg_r, fg_g, fg_b);
                                }
                            }
                            if sx >= dx {
                                let cell = &mut grid[sy * cols + (sx - dx)];
                                if effect.fire_grid[sy * cols + (sx - dx)] == 0 && (cell.ch == ' ' || cell.ch == '─') {
                                    cell.ch = '─';
                                    let fg_r = fade.saturating_add((flare_color.0 as f32 * 0.8) as u8);
                                    let fg_g = ((fade as f32 * 0.75) as u8).saturating_add((flare_color.1 as f32 * 0.8) as u8);
                                    let fg_b = (fade.saturating_add(45)).saturating_add((flare_color.2 as f32 * 0.8) as u8);
                                    cell.fg = (fg_r, fg_g, fg_b);
                                }
                            }
                        }
                    }

                    // Draw vertical flare
                    let v_len = 5;
                    for dy in 1..v_len {
                        let alpha = (90.0f32 * flare_intensity).max(20.0f32) as u8;
                        let fade = alpha.saturating_sub((dy * (80 / v_len)) as u8);
                        if fade > 10 {
                            if sy + dy < rows {
                                let cell = &mut grid[(sy + dy) * cols + sx];
                                if effect.fire_grid[(sy + dy) * cols + sx] == 0 && (cell.ch == ' ' || cell.ch == '│') {
                                    cell.ch = '│';
                                    let fg_r = fade.saturating_add((flare_color.0 as f32 * 0.8) as u8);
                                    let fg_g = ((fade as f32 * 0.75) as u8).saturating_add((flare_color.1 as f32 * 0.8) as u8);
                                    let fg_b = (fade.saturating_add(30)).saturating_add((flare_color.2 as f32 * 0.8) as u8);
                                    cell.fg = (fg_r, fg_g, fg_b);
                                }
                            }
                            if sy >= dy {
                                let cell = &mut grid[(sy - dy) * cols + sx];
                                if effect.fire_grid[(sy - dy) * cols + sx] == 0 && (cell.ch == ' ' || cell.ch == '│') {
                                    cell.ch = '│';
                                    let fg_r = fade.saturating_add((flare_color.0 as f32 * 0.8) as u8);
                                    let fg_g = ((fade as f32 * 0.75) as u8).saturating_add((flare_color.1 as f32 * 0.8) as u8);
                                    let fg_b = (fade.saturating_add(30)).saturating_add((flare_color.2 as f32 * 0.8) as u8);
                                    cell.fg = (fg_r, fg_g, fg_b);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
