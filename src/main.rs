use ccmath::CC;
use sap::{Argument, Parser};
use std::time::Instant;

// A fractal is defined by its function and its clause
#[derive(Clone, Copy, Debug)]
pub struct Fractal {
    pub function: fn(CC<f64>, CC<f64>) -> CC<f64>,
    pub clause: fn(CC<f64>) -> bool,
}

#[derive(Debug, Clone, Copy)]
struct Args {
    fractal: Fractal,
    real_start: f64,
    real_end: f64,
    complex_start: f64,
    complex_end: f64,
    resolution: i64,
    debug: bool,
}

fn iterator(c: [CC<f64>; 2], fractal: Fractal, debug: bool) -> String {
    let mut z0 = c[0];
    let mut z1 = c[1];

    for _ in 1..=36 {
        z0 = (fractal.function)(z0, c[0]);
        z1 = (fractal.function)(z1, c[1]);
    }

    let satisfies = [(fractal.clause)(z0), (fractal.clause)(z1)];

    let mut strng = if satisfies[0] && satisfies[1] {
        String::from("\x1b[34m█\x1b[0m")
    } else if satisfies[0] {
        String::from("\x1b[34m▀\x1b[0m")
    } else if satisfies[1] {
        String::from("\x1b[34m▄\x1b[0m")
    } else {
        String::from(" ")
    };

    if debug {
        strng = String::from("\x1b[41m") + &strng + "\x1b[0m";
    }

    strng
}

fn fractal_matcher(fractal: String) -> Fractal {
    match fractal.as_str() {
        "mandelbrot" => Fractal {
            function: |z, c| z.powi(2) + c,
            clause: |z| z.abs() <= 3f64,
        },
        "mandelbrot-cubed" => Fractal {
            function: |z, c| z.powi(3) + c,
            clause: |z| z.abs() <= 3f64,
        },
        "julia" => Fractal {
            function: |z, _| CC::cos(z),
            clause: |z| z.abs() <= 3f64,
        },
        "bat" => Fractal {
            function: |z, c| CC::arctanh(1f64 / z + 1f64 / c),
            clause: |z| z.abs() <= 3f64,
        },
        "batstatic" => Fractal {
            function: |z, _| CC::cot(1f64 - z.powi(2)),
            clause: |z| z.abs() <= 1f64,
        },
        "spiral" => Fractal {
            function: |z, _| CC::ln(1f64 + z.powi(2)),
            clause: |z| z.abs() <= 1f64,
        },
        "crab" => Fractal {
            function: |z, _| CC::powc(z, 1f64 - z),
            clause: |z| z.abs() <= 1f64,
        },
        // This should never run but sure
        _ => Fractal {
            function: |z, c| CC::arctanh(1f64 / z + 1f64 / c),
            clause: |z| z.abs() <= 1f64,
        },
    }
}

fn main() {
    let mut parser = Parser::from_env().unwrap();
    let mut args = Args {
        fractal: Fractal {
            #[allow(unused)]
            function: |z, c| CC::cos(CC::sqrt(1f64 + z.powi(2))),
            clause: |z| z.abs() <= 1f64,
        },
        real_start: 0f64,
        real_end: 0f64,
        complex_start: 0f64,
        complex_end: 0f64,
        resolution: 1,
        debug: false,
    };

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Long("fractal") => {
                if let Some(fractal) = parser.value() {
                    args.fractal = fractal_matcher(fractal);
                }
            }
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
            Argument::Short('d') => args.debug = true,
            _ => {}
        }
    }

    let now = Instant::now();
    let real_interval = ((args.real_start * args.resolution as f64) as i32)
        ..=((args.real_end * args.resolution as f64) as i32);
    let complex_interval = ((-args.complex_end * args.resolution as f64) as i32)
        ..=((-args.complex_start * args.resolution as f64) as i32);

    for complex in complex_interval.step_by(2) {
        for real in real_interval.clone() {
            let numbers = [
                CC::<f64>::new(real as f64, complex as f64) / args.resolution as f64,
                CC::<f64>::new(real as f64, (complex + 1) as f64) / args.resolution as f64,
            ];
            print!("{}", iterator(numbers, args.fractal, args.debug))
        }
        println!();
    }

    println!(
        "Took {} millis\nTook {} micros\nTook {} nanos",
        now.elapsed().as_millis(),
        now.elapsed().as_micros(),
        now.elapsed().as_nanos()
    );
}
// nya :3
