use num_complex::Complex64;

pub fn mandelbrot() {
    println!("\nlet's make a mandelbrot fractal and save to static/mandelbrot.png\n");

    let max_iter = 200;
    let columns: u32 = 2000;
    let rows: u32 = 1500;

    let re1 = -2.0;
    let re2 = 1.0;
    let dre = (re2 - re1) / columns as f64;
    let im1 = -1.5;
    let im2 = 1.5;
    let dim = (im2 - im1) / rows as f64;

    assert!(re1 < re2);
    assert!(im1 < im2);

    let mut buffer = image::ImageBuffer::new(columns, rows);

    // Set the base color palette of the image.
    // In this case, using a gradient.
    for (col, row, pixel) in buffer.enumerate_pixels_mut() {
        let r = ((col as f32 / columns as f32) * 256.0) as u8;
        let b = 20 + (0.7 * ((rows - row) as f32 / rows as f32) * 256.0) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    for x in 0..columns {
        for y in 0..rows {
            let c = Complex64{re: re1 + ((x as f64) * dre), im: im1 + ((y as f64) * dim)};

            let mut z = c;
            let mut iter = 0;
            while (z.norm_sqr() <= 4.0) && (iter < max_iter) {
                z = c + (z * z);
                iter += 1;
            }

            // println!("{}", i);
            let pixel = buffer.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([0, 50 + ((1.0 * iter as f32)*(y as f32 / rows as f32)) as u8, data[2] as u8]);           
            // println!("{:?}", *pixel);
        }
    }

    // Save format deduced from the path.
    buffer.save("static/mandelbrot.png").unwrap();
}

