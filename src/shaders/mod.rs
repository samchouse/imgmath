pub mod blue_light_filter;

pub trait Shader {
    fn name() -> &'static str;
    fn reverse(&self, pixel: [u8; 3]) -> [u8; 3];
}
