use ccmath::CC;
use sap::{Argument, Parser};

struct Args {
    real_start: f64,
    real_end: f64,
    complex_start: f64,
    complex_end: f64,
    resolution: i64,
}

#[allow(dead_code)]
fn mandelbrot_iterator(c: CC<f64>) -> bool {
    let mut z = c;
    for _ in 1..=36 {
        z = z.powi(2) + c;
    }
    z.abs() <= 3f64
}

fn julia_iterator(c: CC<f64>) -> bool {
    let mut z = c;
    for _ in 1..=36 {
        z = CC::cos(z);
    }
    (z - 0.739085133215160641655312087673).abs() <= 3f64
}

fn main() {
    let mut parser = Parser::from_env().unwrap();
    let mut args = Args {
        real_start: 0f64,
        real_end: 0f64,
        complex_start: 0f64,
        complex_end: 0f64,
        resolution: 1,
    };

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Long("real-start") => {
                if let Some(real_start) = parser.value() {
                    args.real_start = match real_start.parse::<f64>() {
                        Ok(real_start) => real_start,
                        Err(e) => panic!("Invalid argument for real_start: {}", e),
                    };
                }
            }
            Argument::Long("real-end") => {
                if let Some(real_end) = parser.value() {
                    args.real_end = match real_end.parse::<f64>() {
                        Ok(real_end) => real_end,
                        Err(e) => panic!("Invalid argument for real_end: {}", e),
                    };
                }
            }
            Argument::Long("complex-start") => {
                if let Some(complex_start) = parser.value() {
                    args.complex_start = match complex_start.parse::<f64>() {
                        Ok(complex_start) => complex_start,
                        Err(e) => panic!("Invalid argument for complex_start: {}", e),
                    };
                }
            }
            Argument::Long("complex-end") => {
                if let Some(complex_end) = parser.value() {
                    args.complex_end = match complex_end.parse::<f64>() {
                        Ok(complex_end) => complex_end,
                        Err(e) => panic!("Invalid argument for complex_end: {}", e),
                    };
                }
            }
            Argument::Long("resolution") => {
                if let Some(resolution) = parser.value() {
                    args.resolution = match resolution.parse::<i64>() {
                        Ok(resolution) => resolution,
                        Err(e) => panic!("Invalid argument for resolution: {}", e),
                    };
                }
            }
            _ => {}
        }
    }

    let real_interval = ((args.real_start * args.resolution as f64) as i32)
        ..=((args.real_end * args.resolution as f64) as i32);
    let complex_interval = ((-args.complex_end * args.resolution as f64) as i32)
        ..=((-args.complex_start * args.resolution as f64) as i32);

    for complex in complex_interval {
        for real in real_interval.clone() {
            let number = CC::<f64>::new(real as f64, complex as f64) / (args.resolution as f64);
            if julia_iterator(number) {
                print!("\x1b[44m  \x1b[0m");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
