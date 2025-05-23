pub struct kelvin {
    value: f64,
}

impl kelvin {
    fn new(value: f64) -> Self {
        Self { value }
    }

    fn to_celcius(&self) -> f64 {
        self.value - 273.15
    }

    fn to_fahrenheit(&self) -> f64 {
        (self.value - 273.15) * (9.0 / 5.0) + 32.0
    }
}
