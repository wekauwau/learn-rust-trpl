pub struct Celcius {
    value: f64,
}

impl Celcius {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn convert(&self) {
        println!("-----------------------------------------------------");
        println!(
            "{}C = {}F = {}K",
            self.value,
            self.to_fahrenheit(),
            self.to_kelvin()
        );
        println!("-----------------------------------------------------");
    }

    fn to_fahrenheit(&self) -> f64 {
        (self.value * (9.0 / 5.0)) + 32.0
    }

    fn to_kelvin(&self) -> f64 {
        self.value + 273.15
    }
}
