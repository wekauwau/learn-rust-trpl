pub struct Kelvin {
    value: f64,
}

impl Kelvin {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    fn convert(&self) {
        println!("-----------------------------------------------------");
        println!(
            "{}K = {}C = {}F",
            self.value,
            self.to_celcius(),
            self.to_fahrenheit()
        );
        println!("-----------------------------------------------------");
    }

    fn to_celcius(&self) -> f64 {
        self.value - 273.15
    }

    fn to_fahrenheit(&self) -> f64 {
        (self.value - 273.15) * (9.0 / 5.0) + 32.0
    }
}
