// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(pub f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64) // converting u64 to f64
    }
}

impl Duration {
    pub fn as_secs_f64(&self) -> f64 {
        self.0
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }

    fn convert_seconds_to_years(seconds: u64) -> f64 {
        // One year in seconds (approx)
        const SECONDS_IN_YEAR: u64 = 31557600;

        // Convert seconds to Earth years
        (seconds as f64 / (SECONDS_IN_YEAR as f64)) as f64
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
    fn years_during(d: &Duration) -> f64 {
        (d.as_secs_f64() / 31_557_600.0) / 0.2408467
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        (d.as_secs_f64() / 31_557_600.0) / 0.61519726
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.as_secs_f64() / 31_557_600.0 // years to seconds
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        (d.as_secs_f64() / 31_557_600.0) / 1.8808158
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        (d.as_secs_f64() / 31_557_600.0) / 11.862615
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        (d.as_secs_f64() / 31_557_600.0) / 29.447498
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        (d.as_secs_f64() / 31_557_600.0) / 84.016846
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        (d.as_secs_f64() / 31_557_600.0) / 164.79132
    }
}
