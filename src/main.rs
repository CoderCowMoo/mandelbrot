use image::ImageBuffer;
use num_complex::Complex;

fn calculate_val(c: Complex<f64>, max_iter: usize) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    for iter in 0..max_iter {
        if z.norm() > 2.0 {
            return iter;
        }
        z = z * z + c; // x^2 + c
    }
    0
}

// graph mandelbrot set in a .ppm file.
fn main() {
    const IMAGE_WIDTH: usize = 25500;
    const IMAGE_HEIGHT: usize = 25500;

    let mut data = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    let xmin: f64 = -2.0; // xmin and xmax for x axis of mandelbrot set
    let xmax: f64 = 1.0;
    let ymin: f64 = -1.5; // same here
    let ymax: f64 = 1.5;
    let scale_x = (xmax - xmin) / IMAGE_WIDTH as f64; // for adapting it
    let scale_y = (ymax - ymin) / IMAGE_HEIGHT as f64;

    const MAX_ITERATIONS: usize = 240;

    let counter = IMAGE_HEIGHT as f64 / 20.0;

    for (x, y, pixel) in data.enumerate_pixels_mut() {
        // instead of iterating over the x values, we increase it as we move in pixel area
        let cx = xmin + x as f64 * scale_x;
        let cy = ymin + y as f64 * scale_y;

        // remember z0 = x^2 + c
        // here c remains constant because its what we're testing while z changes as it is the iterated value
        let complex_point = Complex::new(cx, cy);

        let colour_lum = calculate_val(complex_point, MAX_ITERATIONS);
        *pixel = image::Luma([colour_lum as u8]);

        // progress counter for possibly long renders
        if x as usize == IMAGE_WIDTH - 1 && (IMAGE_HEIGHT - y as usize - 1) % counter as usize == 0
        {
            println!(
                "{}% left",
                ((IMAGE_HEIGHT - y as usize - 1) as f64 / IMAGE_HEIGHT as f64) * 100.0
            );
        }
    }

    // save img
    data.save("mandelbrot.png").unwrap();

    // das ist finite now
}
