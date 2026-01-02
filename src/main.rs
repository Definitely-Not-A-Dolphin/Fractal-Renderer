use ccmath::CC;
use sap::{Argument, Parser};
use std::time::Instant;

// A fractal is defined by its function and its clause
#[derive(Clone, Copy)]
pub struct Fractal {
    pub function: fn(CC<f64>, CC<f64>) -> CC<f64>,
    pub clause: fn(CC<f64>, CC<f64>) -> bool,
}

struct Args {
    fractal: Fractal,
    real_start: f64,
    real_end: f64,
    imaginary_start: f64,
    imaginary_end: f64,
    iterations: u32,
    resolution: i64,
    debug: bool,
}

fn iterator(numbers: [CC<f64>; 2], fractal: Fractal, debug: bool, iterations: u32) -> String {
    let [c0, c1] = numbers;
    let [mut z0, mut z1] = numbers;

    for _ in 1..=iterations {
        z0 = (fractal.function)(z0, c0);
        z1 = (fractal.function)(z1, c1);
    }

    let (satisfies0, satisfies1) = ((fractal.clause)(z0, c0), (fractal.clause)(z1, c1));

    let strng = if satisfies0 && satisfies1 {
        "\x1b[34m█\x1b[0m"
    } else if satisfies0 {
        "\x1b[34m▀\x1b[0m"
    } else if satisfies1 {
        "\x1b[34m▄\x1b[0m"
    } else {
        " "
    };

    if debug {
        String::from("\x1b[41m") + strng + "\x1b[0m"
    } else {
        String::from(strng)
    }
}

fn fractal_matcher(fractal: String) -> Fractal {
    match fractal.as_str() {
        "mandelbrot" => Fractal {
            function: |z, c| z.powi(2) + c,
            clause: |z, _| z.abs() <= 3f64,
        },
        "mandelbrot-cubed" => Fractal {
            function: |z, c| z.powi(3) + c,
            clause: |z, _| z.abs() <= 3f64,
        },
        "julia" => Fractal {
            function: |z, _| CC::cos(z),
            clause: |z, _| z.abs() <= 3f64,
        },
        "spirals" => Fractal {
            function: |z, _| CC::ln(1f64 + z.powi(2)),
            clause: |z, _| z.abs() <= 1f64,
        },
        "crab" => Fractal {
            function: |z, _| CC::powc(z, 1f64 - z),
            clause: |z, _| z.abs() <= 1f64,
        },
        "singularity" => Fractal {
            function: |z, c| CC::exp(c.powi(2) + z.powi(2)),
            clause: |z, _| z.abs() <= 1f64,
        },
        "singularity-cubed" => Fractal {
            function: |z, c| CC::exp(c.powi(3) + z.powi(3)),
            clause: |z, _| z.abs() <= 1f64,
        },
        _ => Fractal {
            function: |z, c| CC::arctanh(1f64 / z + 1f64 / c),
            clause: |z, _| z.abs() <= 1f64,
        },
    }
}

fn main() {
    let mut parser = Parser::from_env().unwrap();
    let mut args = Args {
        fractal: Fractal {
            function: |z, _| z.powc(-z),
            clause: |z, c| z.abs() <= 1f64 / c.abs(),
        },
        real_start: 0f64,
        real_end: 0f64,
        imaginary_start: 0f64,
        imaginary_end: 0f64,
        iterations: 36u32,
        resolution: 1i64,
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
            Argument::Long("imaginary-start") => {
                if let Some(imaginary_start) = parser.value() {
                    args.imaginary_start = match imaginary_start.parse::<f64>() {
                        Ok(imaginary_start) => imaginary_start,
                        Err(e) => panic!("Invalid argument for complex_start: {}", e),
                    };
                }
            }
            Argument::Long("imaginary-end") => {
                if let Some(imaginary_end) = parser.value() {
                    args.imaginary_end = match imaginary_end.parse::<f64>() {
                        Ok(imaginary_end) => imaginary_end,
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
            Argument::Long("iterations") => {
                if let Some(iterations) = parser.value() {
                    args.iterations = match iterations.parse::<u32>() {
                        Ok(iterations) => iterations,
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
    let imaginary_interval = ((-args.imaginary_end * args.resolution as f64) as i32)
        ..=((-args.imaginary_start * args.resolution as f64) as i32);

    for imaginary in imaginary_interval.step_by(2) {
        for real in real_interval.clone() {
            let real_f64 = real as f64;
            let imaginary_f64 = imaginary as f64;
            let resolution_f64 = args.resolution as f64;
            print!(
                "{}",
                iterator(
                    [
                        CC::<f64>::new(real_f64, imaginary_f64) / resolution_f64,
                        CC::<f64>::new(real_f64, imaginary_f64 + 1f64) / resolution_f64,
                    ],
                    // Cloning is faster, somehow
                    args.fractal.clone(),
                    args.debug,
                    args.iterations
                )
            )
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
