pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


pub fn mandelbrot(width: usize, height: usize, max_iter: i32) -> Vec<u8> {
    // let width = 7680;
    // let height = 4320;
    //let max_iter = 255;

    let mut buffer = vec![0u8; width * height * 3];

    // Region of the complex plane that should always be fully visible
    let (x_min, x_max, y_min, y_max) = (-2.5_f64, 1.0_f64, -1.0_f64, 1.0_f64);
    // Use ONE scale (units per pixel) for both axes so the fractal is never stretched.
    // Take the larger one so the whole region fits; the other axis gets extra margin.
    let scale = f64::max((x_max - x_min) / width as f64, (y_max - y_min) / height as f64);
    let center_x = (x_min + x_max) / 2.0;
    let center_y = (y_min + y_max) / 2.0;

    // Renders a horizontal band of rows starting at `start_row`.
    // Captures only Copy values, so it can be shared freely between threads.
    let render_band = move |band: &mut [u8], start_row: usize| {
                for (y_offset, row) in band.chunks_mut(width * 3).enumerate() {
                    let y = start_row + y_offset;

                    // .chunks_mut(3) yields mutable slices of length 3: [R, G, B]
                    for (x, pixel) in row.chunks_mut(3).enumerate() {
                        let cx = center_x + (x as f64 - width as f64 / 2.0) * scale;
                        let cy = center_y + (y as f64 - height as f64 / 2.0) * scale;

                        let mut zx = 0.0;
                        let mut zy = 0.0;
                        let mut iter = 0;

                        while zx * zx + zy * zy <= 4.0 && iter < max_iter {
                            let tmp = zx * zx - zy * zy + cx;
                            zy = 2.0 * zx * zy + cy;
                            zx = tmp;
                            iter += 1;
                        }

                        // Colorize based on iterations
                        if iter == max_iter {
                            // Points inside the Mandelbrot set remain black
                            pixel[0] = 0; // Red
                            pixel[1] = 0; // Green
                            pixel[2] = 0; // Blue
                        } else {
                            // Map the iteration count to distinct RGB values using modulo
                            pixel[0] = (iter as u32 * 5 % 256) as u8;
                            pixel[1] = (iter as u32 * 7 % 256) as u8;
                            pixel[2] = (iter as u32 * 11 % 256) as u8;
                        }
                    }
                }
    };

    // WebAssembly in the browser (wasm32-unknown-unknown) cannot spawn threads,
    // so render everything on the current thread there.
    #[cfg(target_arch = "wasm32")]
    render_band(&mut buffer, 0);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let num_threads = 8;
        let rows_per_thread = (height / num_threads).max(1);

        std::thread::scope(|s| {
            // Multiply the chunk size by 3 to account for RGB bytes
            for (i, band) in buffer.chunks_mut(width * 3 * rows_per_thread).enumerate() {
                let start_row = i * rows_per_thread;
                s.spawn(move || render_band(band, start_row));
            }
        });
    }

    buffer
}


pub fn save_image(width: usize, height: usize, buffer: &[u8]) {
    // Change ColorType to Rgb8
    image::save_buffer(
        "mandelbrot_color.png",
        buffer,
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    ).expect("Failed to save the image");

    println!("Saved image to mandelbrot_color.png");
}




#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn gen_and_save_image() {
        let img = mandelbrot(4096, 2160, 255);
        save_image(4096, 2160, img.as_slice());
    }

    #[test]
    fn gen_and_save_icon() {
        let img = mandelbrot(910, 512, 255);
        save_image(910, 512, img.as_slice());
    }

    #[test]
    fn get_os() {
        println!("{}", env::consts::OS);
    }

}
