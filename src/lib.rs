use num_complex::Complex64;

// A fractal is defined by its function and its clause
#[derive(Clone, Copy)]
pub struct Fractal {
  pub function: fn(Complex64, Complex64) -> Complex64,
  pub clause: fn(Complex64, Complex64) -> bool,
}

pub struct Args {
  pub fractal: Fractal,
  pub real_start: f64,
  pub real_end: f64,
  pub imaginary_start: f64,
  pub imaginary_end: f64,
  pub iterations: u32,
  pub resolution: i64,
  pub debug: bool,
}
