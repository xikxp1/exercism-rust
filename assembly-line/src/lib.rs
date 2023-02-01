// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::ops::{Mul, Div};

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production = f64::try_from(221 * speed).expect("multiplication error");
    if speed >= 1 && speed <= 4 {
        return production;
    } else if speed <= 8 {
        return 0.9 * production;
    } else if speed <= 10 {
        return 0.77 * production;
    } else {
        panic!("unimplemented");
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60f64) as u32;
}
