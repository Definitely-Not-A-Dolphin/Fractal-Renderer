= Fractals and their function and clause

== Mandelbrot
$
  I(n + 1, c) = I(n, c)^2 + c\
  I(0, c) = c
$

The series should stay bounded.

== Mandelbrot Cubed
$
  I(n + 1, c) = I(n, c)^3 + c\
  I(0, c) = c
$

The series should stay bounded

== Julia
$
  I(n + 1, c) = cos(I(n, c))
  I(0, c) = c
$

The series should stay bounded $<=>$ the series should approach the #link("https://en.wikipedia.org/wiki/Dottie_number")[Dottie number]

== Spirals
$
  I(n + 1, c) = ln(1 + z^2)\
  I(0, c) = c\
$

The should converge to a number with an absolute value smaller or equal to one.

== Crab
$
  I(n + 1, c) = z^(1 - z)\
  I(0, c) = c\
$

The should converge to a number with an absolute value smaller or equal to one.

== Cubed Singularity
$
  I(n + 1, c) = e^(c^2 + z^2)
  I(0, c) = c\
$

The should converge to a number with an absolute value smaller or equal to one.

== Cubed Singularity
$
  I(n + 1, c) = e^(c^3 + z^2)
  I(0, c) = c\
$

The should converge to a number with an absolute value smaller or equal to one.
