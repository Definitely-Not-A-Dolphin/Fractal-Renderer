use fractal_renderer::{Args, DefinedArgs, Fractal};
use num_complex::{Complex64, ComplexFloat};
use sap::{Argument, Parser};
use std::process::exit;
use std::time::Instant;

const HELP_TEXT: &str ="fractal-renderer: Render Fractals in the terminal

Required flags:
  --fractal [string]: \tThe name of the fractal to render. Currently available:
                Mandelbrot, Mandelbrot Cubed, Julia, Spirals, Crab, Singularity, Cubed Singularity.
  --real_start [float]: \tThe starting value of the real part of the interval.
  --real_end [float]: \tThe end value of the real part of the interval.
  --imag_start [float]: \tThe starting value of the imaginary part of the interval.
  --imag_end [float]: \tThe end value of the imaginary part of the interval.

Optional flags:
  --iterations [int]: \tThe amount of iterations to run of each number. Defaults to 36.
  --resolution [int]: \tThe resolution of the rendered fractal. The width and height of the fractal equal the real and imaginary interval length multiplied by the resolution.
  --help: \tShows this text.
  --debug: \tReplaces all the whitespace in a fractal with a color. Useful for finding the optimal resolution.

Shorthands:
  -h\t --help
  -d\t --debug
";
const SEE_HELP_TEXT: &str = ", see the help menu for more information.";

fn print_help() {
  print!("{HELP_TEXT}");
  exit(0);
}

fn iterator(numbers: [Complex64; 2], fractal: Fractal, debug: bool, iterations: u32) -> String {
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

fn fractal_matcher(fractal: String) -> Result<Fractal, ()> {
  match fractal.as_str() {
    "mandelbrot" => Ok(Fractal {
      function: |z, c| z.powi(2) + c,
      clause: |z, _| z.abs() <= 3f64,
    }),
    "mandelbrot-cubed" => Ok(Fractal {
      function: |z, c| z.powi(3) + c,
      clause: |z, _| z.abs() <= 3f64,
    }),
    "julia" => Ok(Fractal {
      function: |z, _| Complex64::cos(z),
      clause: |z, _| z.abs() <= 3f64,
    }),
    "spirals" => Ok(Fractal {
      function: |z, _| Complex64::ln(1f64 + z.powi(2)),
      clause: |z, _| z.abs() <= 1f64,
    }),
    "crab" => Ok(Fractal {
      function: |z, _| Complex64::powc(z, 1f64 - z),
      clause: |z, _| z.abs() <= 1f64,
    }),
    "singularity" => Ok(Fractal {
      function: |z, c| Complex64::exp(c.powi(2) + z.powi(2)),
      clause: |z, _| z.abs() <= 1f64,
    }),
    "singularity-cubed" => Ok(Fractal {
      function: |z, c| Complex64::exp(c.powi(3) + z.powi(3)),
      clause: |z, _| z.abs() <= 1f64,
    }),
    _ => Err(()),
  }
}

fn parse_arguments() -> DefinedArgs {
  let mut parser = Parser::from_env().unwrap();
  let mut args = Args {
    fractal: None,
    real_start: None,
    real_end: None,
    imag_start: None,
    imag_end: None,
    iterations: 36u32,
    resolution: 1u32,
    debug: false,
  };

  while let Some(arg) = parser.forward().unwrap() {
    match arg {
      Argument::Long("fractal") => {
        if let Some(fractal) = parser.value().unwrap() {
          args.fractal = match fractal_matcher(fractal) {
            Ok(fractal) => Some(fractal),
            Err(()) => {
              println!("Invalid fractal choice, see the help menu for more information.");
              exit(0);
            }
          }
        }
      }
      Argument::Long("real-start") => {
        if let Some(real_start) = parser.value().unwrap() {
          args.real_start = match real_start.parse::<f64>() {
            Ok(real_start) => Some(real_start),
            Err(e) => panic!("Invalid argument for real_start: {}", e),
          };
        }
      }
      Argument::Long("real-end") => {
        if let Some(real_end) = parser.value().unwrap() {
          args.real_end = match real_end.parse::<f64>() {
            Ok(real_end) => Some(real_end),
            Err(e) => panic!("Invalid argument for real_end: {}", e),
          };
        }
      }
      Argument::Long("imag-start") => {
        if let Some(imag_start) = parser.value().unwrap() {
          args.imag_start = match imag_start.parse::<f64>() {
            Ok(imag_start) => Some(imag_start),
            Err(e) => panic!("Invalid argument for complex_start: {}", e),
          };
        }
      }
      Argument::Long("imag-end") => {
        if let Some(imag_end) = parser.value().unwrap() {
          args.imag_end = match imag_end.parse::<f64>() {
            Ok(imag_end) => Some(imag_end),
            Err(e) => panic!("Invalid argument for complex_end: {}", e),
          };
        }
      }
      Argument::Long("resolution") => {
        if let Some(resolution) = parser.value().unwrap() {
          args.resolution = match resolution.parse::<u32>() {
            Ok(resolution) => resolution,
            Err(e) => panic!("Invalid argument for resolution: {}", e),
          };
        }
      }
      Argument::Long("iterations") => {
        if let Some(iterations) = parser.value().unwrap() {
          args.iterations = match iterations.parse::<u32>() {
            Ok(iterations) => iterations,
            Err(e) => panic!("Invalid argument for resolution: {}", e),
          };
        }
      }
      Argument::Long("debug") => args.debug = true,
      Argument::Short('d') => args.debug = true,
      Argument::Long("help") => print_help(),
      Argument::Short('h') => print_help(),
      _ => println!("Unrecognised flag{SEE_HELP_TEXT}"),
    }
  }

  if args.fractal.is_none() {
    println!("Missing flag: --fractal{SEE_HELP_TEXT}");
    exit(0);
  }

  if args.real_start.is_none() {
    println!("Missing flag: --real_start{SEE_HELP_TEXT}");
    exit(0);
  }

  if args.real_end.is_none() {
    println!("Missing flag: --real_end{SEE_HELP_TEXT}");
    exit(0);
  }

  if args.imag_start.is_none() {
    println!("Missing flag: --imag_start{SEE_HELP_TEXT}");
    exit(0);
  }

  if args.imag_end.is_none() {
    println!("Missing flag: --imag_end{SEE_HELP_TEXT}");
    exit(0);
  }

  if args.resolution < 2 {
    println!("The resolution must be greater than 2.");
    exit(0);
  }

  DefinedArgs {
    fractal: args.fractal.unwrap(),
    real_start: args.real_start.unwrap(),
    real_end: args.real_end.unwrap(),
    imag_start: args.imag_start.unwrap(),
    imag_end: args.imag_end.unwrap(),
    iterations: args.iterations,
    resolution: args.resolution,
    debug: args.debug,
  }
}

fn main() {
  let args = parse_arguments();

  let now = Instant::now();
  let real_interval = ((args.real_start * args.resolution as f64) as i32)
    ..=((args.real_end * args.resolution as f64) as i32);
  let imag_interval = ((-args.imag_end * args.resolution as f64) as i32)
    ..=((-args.imag_start * args.resolution as f64) as i32);

  for imag in imag_interval.step_by(2) {
    for real in real_interval.clone() {
      let real_f64 = real as f64;
      let imag_f64 = imag as f64;
      let resolution_f64 = args.resolution as f64;
      print!(
        "{}",
        iterator(
          [
            Complex64::new(real_f64, imag_f64) / resolution_f64,
            Complex64::new(real_f64, imag_f64 + 1f64) / resolution_f64,
          ],
          args.fractal,
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
