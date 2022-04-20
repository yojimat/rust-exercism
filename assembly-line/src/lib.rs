// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

const CARS_PER_HOUR: f64 = 221.0;

fn success_rate_by_speed(speed: u8) -> i8 {
    match speed {
        0 => 100,
        1..=4 => 100,
        5..=8 => 90,
        9..=10 => 77,
        _ => 0,
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed_float = speed as f64;
    let cars_speed_hour = CARS_PER_HOUR * speed_float;
    cars_speed_hour * success_rate_by_speed(speed) as f64 / 100.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
