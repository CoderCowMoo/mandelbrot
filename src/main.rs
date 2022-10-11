use num_complex::Complex;
use std::{fs::File, io::Write};

// graph mandelbrot set in a .ppm file.
fn main() {
    const IMAGE_WIDTH: usize = 500;
    const IMAGE_HEIGHT: usize = 500;

    let data: Vec<u8> = vec![255, 0, 0, 0, 255, 0, 0, 0, 255];

    // instantiate ppm file
    let mut out_file = match File::create("mandelbrot.ppm") {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open file mandelbrot.ppm because: {}", e),
    };

    writeln!(out_file, "P3")
        .map_err(|e| panic!("Failed writing to file: {}", e))
        .unwrap();
    writeln!(out_file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap(); // safe to unwrap because we checked for error in first write
    writeln!(out_file, "255").unwrap();
    for i in data.chunks(3) {
        write!(out_file, "{} {} {}\n", i[0], i[1], i[2]).unwrap();
    }

    // das ist finite now
}
