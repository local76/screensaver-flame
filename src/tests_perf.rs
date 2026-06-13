use std::time::{Duration, Instant};
use crate::flame::Flame;
use crate::runner::core::TerminalCell;
use crate::runner::core::screensaver::Screensaver;

#[test]
fn test_performance_100_frames() {
    let mut flame = Flame::new();
    // Tip: set sys_refresh_timer = -1000.0 to prevent slow system info calls in tests
    flame.sys_refresh_timer = -1000.0;

    let cols = 80;
    let rows = 24;
    let mut grid = vec![TerminalCell::default(); cols * rows];

    let start = Instant::now();

    for _ in 0..100 {
        flame.update(Duration::from_millis(16), cols, rows);
        flame.draw(&mut grid, cols, rows);
    }

    let duration = start.elapsed();
    println!("100 frames completed in {:?}", duration);

    // Assert it completes within a budget of 1500ms
    assert!(
        duration < Duration::from_millis(1500),
        "Performance budget exceeded: {:?}",
        duration
    );
}
