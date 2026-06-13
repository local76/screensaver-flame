use super::*;
use std::time::Duration;
use crate::flame::{Spark, VolcanicGlob};
use crate::runner::core::screensaver::Screensaver;
use crate::runner::core::LcgRng;

#[test]
fn test_get_palette_corners() {
    let accent = (0, 128, 255);
    let palette = get_palette(accent);
    // Index 0 must always be black
    assert_eq!(palette[0], (0, 0, 0));
    // Index 35 is white-hot, so it should have maximum values
    assert_eq!(palette[35].0, 255);
    assert!(palette[35].1 >= 240);
    assert!(palette[35].2 >= 200);

    // Let's verify monotonicity of red component or total intensity
    for i in 1..36 {
        let (r, g, b) = palette[i];
        let intensity = r as u32 + g as u32 + b as u32;
        let (r_prev, g_prev, b_prev) = palette[i - 1];
        let intensity_prev = r_prev as u32 + g_prev as u32 + b_prev as u32;
        // The overall intensity should generally be non-decreasing (going from black to white-hot)
        assert!(intensity >= intensity_prev, "Intensity decreased at index {}", i);
    }
}

#[test]
fn test_rng_math() {
    let mut rng = LcgRng::new(12345);
    
    // Test float range
    for _ in 0..100 {
        let val = rng.next_range(2.5, 7.5);
        assert!(val >= 2.5 && val <= 7.5);
    }

    // Test next_f32
    for _ in 0..100 {
        let val = rng.next_f32();
        assert!(val >= 0.0 && val <= 1.0);
    }

    // Test next_usize
    for _ in 0..100 {
        let val = rng.next_usize(10);
        assert!(val < 10);
    }

    // Test next_bool distribution roughly
    let mut trues = 0;
    for _ in 0..1000 {
        if rng.next_bool(0.3) {
            trues += 1;
        }
    }
    // 300 expected, check broad range
    assert!(trues > 200 && trues < 400);
}

#[test]
fn test_step_fire_math() {
    let mut flame = Flame::new();
    flame.sys_refresh_timer = -1000.0;
    let cols = 10;
    let rows = 5;
    flame.fire_grid = vec![0; cols * rows];
    flame.cpu_load = 0.5;
    flame.mem_pressure = 0.5;

    // Step once to initialize bottom row
    step_fire(&mut flame, cols, rows);

    // Verify bottom row has heat (value > 0)
    let bottom_start = (rows - 1) * cols;
    for x in 0..cols {
        assert!(flame.fire_grid[bottom_start + x] > 0);
    }

    // Verify upper rows are still mostly cold or propagation has started
    // Let's step enough times for heat to propagate up to the top
    for _ in 0..10 {
        step_fire(&mut flame, cols, rows);
    }

    // The top row (row 0) should now have some heat
    let mut top_heat = 0;
    for x in 0..cols {
        top_heat += flame.fire_grid[x] as u32;
    }
    assert!(top_heat > 0, "Heat did not propagate to the top row");
}

#[test]
fn test_spark_physics_wind_math() {
    let mut flame = Flame::new();
    flame.sys_refresh_timer = -1000.0;
    // Call update once to handle initial resize to 40x20
    flame.update(Duration::from_millis(16), 40, 20);

    // Now manually add a spark
    flame.sparks.push(Spark {
        x: 10.0,
        y: 10.0,
        vx: 0.0,
        vy: -5.0,
        life: 2.0,
        max_life: 2.0,
    });

    // Step again
    flame.update(Duration::from_millis(100), 40, 20);

    assert_eq!(flame.sparks.len(), 1);
    let spark = &flame.sparks[0];
    // y should decrease (since vy is negative)
    assert!(spark.y < 10.0);
    // life should decay
    assert!(spark.life < 2.0);
    // vx should change from 0.0 due to wind acceleration
    assert_ne!(spark.vx, 0.0);
}

#[test]
fn test_volcanic_glob_gravity_math() {
    let mut flame = Flame::new();
    flame.sys_refresh_timer = -1000.0;
    // Call update once to handle initial resize to 40x25
    flame.update(Duration::from_millis(16), 40, 25);

    // Manually add a glob
    flame.volcanic_globs.push(VolcanicGlob {
        x: 5.0,
        y: 20.0,
        vx: 2.0,
        vy: -10.0,
        life: 4.5,
    });

    flame.update(Duration::from_millis(100), 40, 25);

    assert_eq!(flame.volcanic_globs.len(), 1);
    let glob = &flame.volcanic_globs[0];
    // Gravity is 12.0, so vy should increase (become less negative)
    // vy_new = vy_old + gravity * dt = -10.0 + 12.0 * 0.1 = -8.8
    assert!((glob.vy - (-8.8)).abs() < 1e-4, "Expected vy around -8.8, got {}", glob.vy);
    // x should increase by vx * dt = 2.0 * 0.1 = 0.2
    assert!((glob.x - 5.2).abs() < 1e-4, "Expected x around 5.2, got {}", glob.x);
    // life should decay by 0.1
    assert!((glob.life - 4.4).abs() < 1e-4);
}
