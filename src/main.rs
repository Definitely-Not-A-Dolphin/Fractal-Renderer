use args::parse_arguments;
use fractal_renderer::Fractal;
use num_complex::Complex64;
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
  let resolution = args.resolution as f64;
  let now = Instant::now();
  let real_interval =
    ((args.real_start * resolution) as i32)..=((args.real_end * resolution) as i32);
  let imag_interval =
    ((-args.imag_end * resolution) as i32)..=((-args.imag_start * resolution) as i32);

  for imag in imag_interval.step_by(2) {
    for real in real_interval.clone() {
      let real = real as f64;
      let imag = imag as f64;
      print!(
        "{}",
        iterator(
          [
            Complex64::new(real, imag) / resolution,
            Complex64::new(real, imag + 1f64) / resolution
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
    "Took {} millis
    Took {} micros
    Took {} nanos",
    now.elapsed().as_millis(),
    now.elapsed().as_micros(),
    now.elapsed().as_nanos()
  );
}
// nya :3
