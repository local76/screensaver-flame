use super::*;
use std::time::Duration;
use crate::runner::core::TerminalCell;

#[test]
fn test_flame_new() {
    let mut flame = Flame::new();
    // Tip: set sys_refresh_timer = -1000.0 if applicable to prevent slow system info calls in tests
    flame.sys_refresh_timer = -1000.0;
    assert_eq!(flame.last_cols, 0);
    assert_eq!(flame.last_rows, 0);
    assert_eq!(flame.sparks.len(), 0);
}

#[test]
fn test_flame_update_and_draw() {
    let mut flame = Flame::new();
    // Tip: set sys_refresh_timer = -1000.0 if applicable to prevent slow system info calls in tests
    flame.sys_refresh_timer = -1000.0;
    flame.update(Duration::from_millis(16), 80, 24);
    let mut grid = vec![TerminalCell::default(); 80 * 24];
    flame.draw(&mut grid, 80, 24);
    // Ensure state variables get initialized
    assert_eq!(flame.last_cols, 80);
    assert_eq!(flame.last_rows, 24);
    assert!(!flame.stars.is_empty());
}
