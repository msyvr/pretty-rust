mod fractal;
mod mandelbrot;
fn main() {
    println!("\npretty things incoming...\n");
    // Generate a julia fractal.
    fractal::fractal();
    // Generate a mandelbrot fractal.
    mandelbrot::mandelbrot();
}
