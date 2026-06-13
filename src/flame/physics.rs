//! Physics, helpers, and drawing calculations for the flame screensaver.

use super::Flame;

/// Generates a fire heat ramp palette based on the given accent color.
pub fn get_palette(_accent: (u8, u8, u8)) -> [(u8, u8, u8); 36] {
    let mut palette = [(0, 0, 0); 36];
    palette[0] = (0, 0, 0);
    for (i, color) in palette.iter_mut().enumerate().skip(1) {
        if i < 12 {
            // Dark red to bright red
            let t = i as f32 / 12.0;
            *color = (
                (200.0 * t) as u8,
                0,
                0,
            );
        } else if i < 24 {
            // Bright red to vibrant orange/gold
            let t = (i - 12) as f32 / 12.0;
            *color = (
                (200.0 + 55.0 * t) as u8,
                (140.0 * t) as u8,
                0,
            );
        } else if i < 32 {
            // Orange/gold to bright yellow
            let t = (i - 24) as f32 / 8.0;
            *color = (
                255,
                (140.0 + 90.0 * t) as u8,
                (50.0 * t) as u8,
            );
        } else {
            // Bright yellow to white-hot
            let t = (i - 32) as f32 / 3.0;
            *color = (
                255,
                (230.0 + 25.0 * t) as u8,
                (50.0 + 190.0 * t) as u8,
            );
        }
    }
    palette
}

/// Computes the next step of the cellular automata representing the fire.
pub fn step_fire(flame: &mut Flame, cols: usize, rows: usize) {
    // 1. Maintain bottom row (fire source) with dynamic flicker
    let bottom_row_start = (rows - 1) * cols;
    let heat_base = 26.0 + flame.cpu_load * 9.0 + flame.mem_pressure * 7.0;
    for x in 0..cols {
        let idx = bottom_row_start + x;
        flame.fire_grid[idx] = (flame.rng.next_range(heat_base, heat_base + 13.0) as u8).min(35);
    }

    // Slightly seed the second row from the bottom to keep the fire thick
    if rows > 2 {
        let second_bottom_start = (rows - 2) * cols;
        for x in 0..cols {
            let idx = second_bottom_start + x;
            if flame.rng.next_bool(0.7) {
                flame.fire_grid[idx] = (flame.rng.next_range(26.0, 36.0) as u8).min(35);
            }
        }
    }

    // Occasional large fire plumes
    if rows >= 3 && flame.rng.next_bool(0.12) {
        let flare_width = flame.rng.next_range(3.0, 8.0) as usize;
        let flare_x = flame.rng.next_usize(cols.saturating_sub(flare_width));
        let bottom_row = rows - 1;
        for dx in 0..flare_width {
            let x = flare_x + dx;
            if x < cols {
                for dy in 0..3 {
                    let y = bottom_row - dy;
                    let idx = y * cols + x;
                    flame.fire_grid[idx] = 35;
                }
            }
        }
    }

    // 2. Propagate fire upwards
    for y in 1..rows {
        for x in 0..cols {
            let src_idx = y * cols + x;
            let fire_val = flame.fire_grid[src_idx];

            if fire_val == 0 {
                let rand_x = flame.rng.next_range(-1.0, 2.0) as i32;
                let dst_x = (x as i32 + rand_x).clamp(0, cols as i32 - 1) as usize;
                let dst_y = y - 1;
                flame.fire_grid[dst_y * cols + dst_x] = 0;
            } else {
                let height_ratio = (rows - 1 - y) as f32 / rows as f32;
                let min_decay = if height_ratio > 0.65 { 1.6 } else if height_ratio > 0.4 { 1.0 } else { 0.4 };
                let max_decay = if height_ratio > 0.65 { 3.4 } else if height_ratio > 0.4 { 2.4 } else { 1.7 };
                let decay_mult = match flame.flame_height_opt {
                    0 => 3.5f32,
                    2 => 1.3f32,
                    _ => 2.2f32,
                };
                let decay = ((flame.rng.next_range(min_decay, max_decay) * decay_mult) as u8).max(1);

                let rand_x = flame.rng.next_range(-1.0, 2.0) as i32;
                let dst_x = (x as i32 + rand_x).clamp(0, cols as i32 - 1) as usize;
                let dst_y = y - 1;

                flame.fire_grid[dst_y * cols + dst_x] = fire_val.saturating_sub(decay);
            }
        }
    }
}

#[cfg(test)]
#[path = "physics_tests.rs"]
mod tests;
