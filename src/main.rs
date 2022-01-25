use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tcfg: TempConfig = TempConfig::new(&args).unwrap_or_else(|err| {
        eprintln!("Argument error: {}", err);
        eprintln!("Usage:\n  celfar [TEMP (1/1.0)] [UNIT (c/f)]");
        std::process::exit(1);
    });

    println!("{}", tcfg.temp_conv().round());
}

enum Unit {
    Celsius,
    Fahrenheit,
}

struct TempConfig {
    temp: f64,
    unit: Unit,
}

impl TempConfig {
    fn new(args: &[String]) -> Result<TempConfig, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let temp: f64 = match args[1].parse() {
            Ok(n) => n,
            Err(_) => return Err("invalid temp"),
        };
        let unit: Unit = match args[2].to_lowercase().as_str() {
            "f" => Unit::Fahrenheit,
            "c" => Unit::Celsius,
            _ => return Err("invalid unit"),
        };

        Ok(TempConfig { temp, unit })
    }

    fn temp_conv(&self) -> f64 {
        match self.unit {
            Unit::Fahrenheit => (self.temp * 1.8) + 32.0,
            Unit::Celsius => (self.temp - 32.0) / 1.8,
        }
    }
}
