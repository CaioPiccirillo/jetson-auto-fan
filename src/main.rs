use clap::{AppSettings, Clap};
use std::fs;
use std::{thread, time};

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Initialize the control
    #[clap(short, long)]
    init: bool,
    /// Level of verbosity
    #[clap(short, long, default_value = "1")]
    verbose: i32,
}

const THERMAL_ZONE: &str = "/sys/devices/virtual/thermal/thermal_zone0/temp";
const TARGET_PWM: &str = "/sys/devices/pwm-fan/target_pwm";
const LOW_LIMIT: usize = 25_000;
const HIGH_LIMIT: usize = 40_000;

fn main() {
    let opts: Opts = Opts::parse();
    let mut temp: usize = 0;
    if opts.init {
        loop {
            let sleep_duration = time::Duration::from_secs(10);
            let contents = fs::read_to_string(THERMAL_ZONE)
                .expect("Something went wrong reading the temperature");
            let current_temp = contents.trim().parse::<usize>().unwrap();
            if current_temp != temp {
                println!("Temperature: {}", current_temp);
                let pwm = map_fan_pwm(current_temp);
                println!("PWM: {}", pwm);
                fs::write(TARGET_PWM, pwm.to_string())
                    .expect("Something went wrong writing to file");
            }
            temp = current_temp;
            thread::sleep(sleep_duration);
        }
    }

    // match opts.verbose {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     _ => println!("Don't be ridiculous"),
    // }
}

fn map_fan_pwm(temp: usize) -> usize {
    match temp {
        t if t < LOW_LIMIT => 0,
        t if t > HIGH_LIMIT => 255,
        _ => (255 * (temp - LOW_LIMIT)) / (HIGH_LIMIT - LOW_LIMIT),
    }
}
