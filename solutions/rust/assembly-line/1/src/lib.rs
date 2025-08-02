// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate: f64 = if (speed > 8) { 0.77 } else if (speed > 4) { 0.9 } else { 1.0 };
    rate * 221.0 * (speed as f64)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}
