use ccmath::Complex;

fn mandelbrot(z: Complex, c: Complex) -> Complex {
    z.powi(2) + c
}

fn iterator(c: Complex) -> bool {
    let mut z = c;
    for _ in 1..=36 {
        z = mandelbrot(z, c);
        //println!("{:?}", z);
    }
    if z.abs() <= 3f64 {
        return true;
    }
    return false;
}

fn main() {
    for complex in -10..=10 {
        for real in -20..=5 {
            let number = Complex::new(real as f64 / 10f64, complex as f64 / 10f64);
            if iterator(number) {
                print!("\x1b[44m   \x1b[0m");
            } else {
                print!("\x1b[41m   \x1b[0m");
            }
        }
        print!("\n");
    }
}
