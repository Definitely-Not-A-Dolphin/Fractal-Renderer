use num_complex::Complex64;

// A fractal is defined by its function and its clause
#[derive(Clone, Copy)]
pub struct Fractal {
  pub function: fn(Complex64, Complex64) -> Complex64,
  pub clause: fn(Complex64, Complex64) -> bool,
}

pub struct Args {
  pub fractal: Option<Fractal>,
  pub real_start: Option<f64>,
  pub real_end: Option<f64>,
  pub imag_start: Option<f64>,
  pub imag_end: Option<f64>,
  pub iterations: u32,
  pub resolution: u32,
  pub debug: bool,
}

pub struct DefinedArgs {
  pub fractal: Fractal,
  pub real_start: f64,
  pub real_end: f64,
  pub imag_start: f64,
  pub imag_end: f64,
  pub iterations: u32,
  pub resolution: u32,
  pub debug: bool,
}
