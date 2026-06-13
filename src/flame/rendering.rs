use crate::runner::core::TerminalCell;
use crate::runner::toolkit::sys_info::query_current_palette;
use super::Flame;
use super::stars::draw_stars_and_flares;

pub fn draw_fire(effect: &Flame, grid: &mut [TerminalCell], cols: usize, rows: usize) {
    const CHARS: &[char] = &[
        ' ', '.', ':', '-', '=', '+', '*', 'o', 's', 'x', 'z', '#', 'A', '@', '█'
    ];

    // Clear the grid and draw stars & flares
    draw_stars_and_flares(effect, grid, cols, rows);

    // 2. Render fire grid (overlays stars/flares where fire_val > 0)
    for y in 0..rows {
        for x in 0..cols {
            let mut fire_val = effect.fire_grid[y * cols + x] as usize;
            if fire_val > 0 {
                fire_val = fire_val.min(35);
                let char_idx = (fire_val * (CHARS.len() - 1)) / 35;
                let ch = CHARS[char_idx];
                let fg = effect.palette[fire_val];

                grid[y * cols + x] = TerminalCell {
                    ch,
                    fg,
                    bg: (0, 0, 0),
                    bold: fire_val > 14,
                };
            }
        }
    }

    // 3. Overlay rising sparks
    for spark in &effect.sparks {
        let sx = spark.x.round() as i32;
        let sy = spark.y.round() as i32;
        if sx >= 0 && sx < cols as i32 && sy >= 0 && sy < rows as i32 {
            let ux = sx as usize;
            let uy = sy as usize;
            let grid_idx = uy * cols + ux;

            let life_pct = spark.life / spark.max_life;
            let ch = if life_pct > 0.72 {
                '*'
            } else if life_pct > 0.32 {
                '+'
            } else {
                '.'
            };

            let color = if life_pct > 0.75 {
                let t = (life_pct - 0.75) / 0.25;
                (
                    255,
                    (180.0 + 75.0 * t) as u8,
                    (120.0 * t) as u8,
                )
            } else if life_pct > 0.35 {
                let t = (life_pct - 0.35) / 0.40;
                (
                    (180.0 + 75.0 * t) as u8,
                    (t * 180.0) as u8,
                    0,
                )
            } else {
                let t = life_pct / 0.35;
                (
                    (180.0 * t) as u8,
                    0,
                    0,
                )
            };

            let current = &mut grid[grid_idx];
            let current_fire_val = effect.fire_grid[grid_idx];
            if current_fire_val < 10 {
                current.ch = ch;
                current.fg = color;
                current.bold = life_pct > 0.45;
            }
        }
    }

    // 3.5. Overlay active volcanic globs (100% larger with core and envelope)
    for glob in &effect.volcanic_globs {
        let gx = glob.x.round() as i32;
        let gy = glob.y.round() as i32;
        
        let cells = [
            (gx, gy, '●', (255, 255, 200), true),      // Core
            (gx - 1, gy, 'o', (255, 130, 0), true),     // Left
            (gx + 1, gy, 'o', (255, 130, 0), true),     // Right
            (gx, gy - 1, 'o', (255, 130, 0), true),     // Top
            (gx, gy + 1, 'o', (255, 130, 0), true),     // Bottom
        ];

        for &(px, py, ch, fg, bold) in &cells {
            if px >= 0 && px < cols as i32 && py >= 0 && py < rows as i32 {
                let grid_idx = py as usize * cols + px as usize;
                grid[grid_idx] = TerminalCell {
                    ch,
                    fg,
                    bg: (0, 0, 0),
                    bold,
                };
            }
        }
    }

    // 4. Draw logo cells (styled with Windows Theme Accent color)
    let theme_accent = query_current_palette().accent;
    for cell in &effect.logo_cells {
        if cell.x >= cols || cell.y >= rows {
            continue;
        }
        let grid_idx = cell.y * cols + cell.x;
        let temp = cell.temp.min(1.0);

        // library 4.0: pull the accent per-frame from the canonical
        // ScreenPalette. Replaces the pre-4.0 `effect.theme_accent` field
        // so OS theme changes propagate without restarting the saver.
        let mut fg = theme_accent;
        if temp > 0.1 {
            let t = (temp - 0.1) / 0.9;
            fg.0 = (fg.0 as f32 * (1.0 - t) + 255.0 * t) as u8;
            fg.1 = (fg.1 as f32 * (1.0 - t) + 255.0 * t) as u8;
            fg.2 = (fg.2 as f32 * (1.0 - t) + 180.0 * t) as u8;
        }

        grid[grid_idx] = TerminalCell {
            ch: cell.ch,
            fg,
            bg: (0, 0, 0),
            bold: temp > 0.15,
        };
    }
}
