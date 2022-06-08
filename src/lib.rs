pub fn write_sample_image() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let r = x as f32 / (image_width as f32 - 1.0);
            let g = y as f32 / (image_height as f32 - 1.0);
            let b = 0.25 as f32;

            print!(
                "{} {} {}\n",
                (r * 255.999).round() as i32,
                (g * 255.999).round() as i32,
                (b * 255.999).round() as i32
            );
        }
    }
}
