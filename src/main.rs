use fractal_renderer::Fractal;
use num_complex::Complex64;
use args::parse_arguments;
use std::time::Instant;

mod args;

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
