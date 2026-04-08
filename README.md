# Fractal-Renderer

Render fractials in the terminal

## How to run the program

```bash
cargo run --
  --real-start=<start of real interval>
  --real-end=<end of real interval>
  --imaginary-start=<start of imaginary interval>
  --imaginary-end=<end of imaginary interval>
  --resolution=<resolution of image>
  --iterations=<amount of times the function should be iterated>
  --fractal=<fractal name>
```

`resolution` will increase the size of the image, so in order to maintain the same size, use this rule of thumb: when multiplying the `resolution` with a factor of $n$, divide the values of the intervals by $n$.

## Available fractals

### Mandelbrot

$$
I(n + 1, c) = I(n, c)^2 + c \newline
I(0, c) = c
$$

The series should stay bounded.

### Mandelbrot Cubed

$$
I(n + 1, c) = I(n, c)^3 + c \newline
I(0, c) = c
$$

The series should stay bounded

### Julia

$$
I(n + 1, c) = \cos(I(n, c)) \newline
I(0, c) = c
$$

The series should stay bounded $<=>$ the series should approach the [The Dottie Number](https://en.wikipedia.org/wiki/Dottie_number)

### Spirals

$$
I(n + 1, c) = \ln(1 + z^2) \newline
I(0, c) = c
$$

The should converge to a number with an absolute value smaller or equal to one.

### Crab

$$
I(n + 1, c) = z^{1 - z} \newline
I(0, c) = c
$$

The should converge to a number with an absolute value smaller or equal to one.

### Singularity

$$
I(n + 1, c) = e^{c^2 + z^2} \newline
I(0, c) = c
$$

The should converge to a number with an absolute value smaller or equal to one.

### Cubed Singularity

$$
I(n + 1, c) = e^{c^3 + z^3} \newline
I(0, c) = c
$$

The should converge to a number with an absolute value smaller or equal to one.
