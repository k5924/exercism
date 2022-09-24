// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed <= 4 {
        f64::from(speed) * 221.0
    } else if speed > 4 && speed < 9 {
        f64::from(speed) * 221.0 * 0.9
    } else {
        f64::from(speed) * 221.0 * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return production_rate_per_hour(speed) as u32 / 60
}
