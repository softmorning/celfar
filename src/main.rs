use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tcfg: TempConfig = TempConfig::new(&args).unwrap_or_else(|err| {
        eprintln!("Argument error: {}", err);
        eprintln!("Usage:\n  celfar [TEMP (1/1.0)] [UNIT (c/f)]");
        std::process::exit(1);
    });

    println!("{}", temp_conv(&tcfg).round());
}

struct TempConfig {
    temp: f64,
    unit: String,
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
        let unit: String = match args[2].to_lowercase().as_str() {
            "f" => "f".to_string(),
            "c" => "c".to_string(),
            _ => return Err("invalid unit"),
        };

        Ok(TempConfig { temp, unit })
    }
}

fn temp_conv(cfg: &TempConfig) -> f64 {
    match cfg.unit.as_str() {
        "f" => (cfg.temp * 1.8) + 32.0,
        "c" => (cfg.temp - 32.0) / 1.8,
        _ => 0.0 // This never happens and my rust knowledge is not very extensive, no clue if this is a good practice or not. Gut says no.
    }
}
