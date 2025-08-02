#[derive(Debug)]
pub struct Duration(f64);

impl Duration {
    const SECONDS_PER_EARTH_YEAR: f64 = 365.25_f64 * 86400_f64;
}

impl From<u64> for Duration {    
    fn from(s: u64) -> Self {
        Duration(s as f64 / Self::SECONDS_PER_EARTH_YEAR)
    }
}

pub trait Planet {
     fn year_duration() -> f64;
    
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::year_duration()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn year_duration() -> f64 { 0.2408467 }
}
impl Planet for Venus {
     fn year_duration() -> f64 { 0.61519726 }
}
impl Planet for Earth {
     fn year_duration() -> f64 { 1.0 }
}
impl Planet for Mars {
     fn year_duration() -> f64 { 1.8808158 }
}
impl Planet for Jupiter {
     fn year_duration() -> f64 { 11.862615 }
}
impl Planet for Saturn {
     fn year_duration() -> f64 { 29.447498 }
}
impl Planet for Uranus {
     fn year_duration() -> f64 { 84.016846 }
}
impl Planet for Neptune {
     fn year_duration() -> f64 { 164.79132 }
}
