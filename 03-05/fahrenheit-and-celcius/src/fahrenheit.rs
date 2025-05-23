pub struct Fahrenheit {
    value: f64,
}

impl Fahrenheit {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    fn convert(&self) {
        println!("-----------------------------------------------------");
        println!(
            "{}F = {}C = {}K",
            self.value,
            self.to_celcius(),
            self.to_kelvin()
        );
        println!("-----------------------------------------------------");
    }

    fn to_celcius(&self) -> f64 {
        (self.value - 32.0) * (5.0 / 9.0)
    }

    fn to_kelvin(&self) -> f64 {
        (self.value - 32.0) * (5.0 / 9.0)
    }
}
