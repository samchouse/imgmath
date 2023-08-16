pub mod blue_light_filter;

pub trait Shader {
    fn name() -> &'static str;
    fn reverse(&self, pixel: [u8; 3]) -> [u8; 3];
    fn reverse_on_file(&self, input: &str, output: &str) {
        let mut img = image::open(input).unwrap().to_rgba8();
        img.pixels_mut().for_each(|pixel| {
            let reversed_pixel = self.reverse([pixel[0], pixel[1], pixel[2]]);
            *pixel = image::Rgba([
                reversed_pixel[0],
                reversed_pixel[1],
                reversed_pixel[2],
                pixel[3],
            ]);
        });
        img.save(output).unwrap();
    }
}
