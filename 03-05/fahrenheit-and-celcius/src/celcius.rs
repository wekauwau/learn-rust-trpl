pub struct celcius {
    value: f64,
}

impl celcius {
    fn new(value: f64) -> Self {
        Self { value }
    }

    fn to_fahrenheit(&self) -> f64 {
        (self.value * (9.0 / 5.0)) + 32.0
    }

    fn to_kelvin(&self) -> f64 {
        self.value + 273.15
    }
}
