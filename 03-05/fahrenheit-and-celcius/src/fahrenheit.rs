pub struct fahrenheit {
    value: f64,
}

impl fahrenheit {
    fn new(value: f64) -> Self {
        Self { value }
    }

    fn to_celcius(&self) -> f64 {
        (self.value - 32.0) * (5.0 / 9.0)
    }

    fn to_kelvin(&self) -> f64 {
        (self.value - 32.0) * (5.0 / 9.0)
    }
}
