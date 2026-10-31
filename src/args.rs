use fractal_renderer::{Args, DefinedArgs, Fractal};
use num_complex::{Complex64, ComplexFloat};
use sap::{Argument, Parser};
use std::process::exit;

const HELP_TEXT: &str ="fractal-renderer: Render Fractals in the terminal

Required flags:
  --fractal\t[string]: \tThe name of the fractal to render. Currently available:
                Mandelbrot, Mandelbrot Cubed, Julia, Spirals, Crab, Singularity, Cubed Singularity.
  --real_start\t[float]: \tThe starting value of the real part of the interval.
  --real_end\t[float]: \tThe end value of the real part of the interval.
  --imag_start\t[float]: \tThe starting value of the imaginary part of the interval.
  --imag_end\t[float]: \tThe end value of the imaginary part of the interval.

Optional flags:
  --iterations\t[int]: \tThe amount of iterations to run of each number. Defaults to 36.
  --resolution\t[int]: \tThe resolution of the rendered fractal. The width and height of the fractal equal the real and imaginary interval length multiplied by the resolution.
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

pub fn parse_arguments() -> DefinedArgs {
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
