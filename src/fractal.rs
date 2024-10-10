pub fn fractal() {
    println!("\nlet's make a jula fractal and save to static/fractal.png\n");

    // Generate a julia fractal.
    let columns = 1500;
    let rows = 1500;

    let k_cols = 3.0 / columns as f32;
    let k_rows = 3.0 / rows as f32;

    let mut buffer = image::ImageBuffer::new(columns, rows);

    // Set the base color palette of the image.
    // In this case, using a gradient.
    for (col, row, pixel) in buffer.enumerate_pixels_mut() {
        let r = (0.2 * col as f32) as u8;
        let b = (0.2 * row as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    for x in 0..columns {
        for y in 0..rows {
            let cx = y as f32 * k_cols - 1.5;
            let cy = x as f32 * k_rows - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            // println!("{}", i);
            let pixel = buffer.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);           
            // println!("{:?}", *pixel);
        }
    }

    // Save format deduced from the path.
    buffer.save("static/fractal.png").unwrap();
}

