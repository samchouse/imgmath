use nalgebra_glm::*;

use super::Shader;

pub struct BlueLightFilter {
    temperature: f64,
}

impl BlueLightFilter {
    pub fn new(temperature: f64) -> Self {
        Self { temperature }
    }

    fn color_temperature_to_rgb(&self) -> TVec3<f64> {
        let m = mat3(
            0.0,
            -2902.1955373783176,
            -8257.7997278925690,
            0.0,
            1669.5803561666639,
            2575.2827530017594,
            1.0,
            1.3302673723350029,
            1.8993753891711275,
        );

        let a = clamp_scalar(self.temperature, 1000.0, 40000.0);
        let b = vec3(a, a, a);
        let c = b + vec3(m.m21, m.m22, m.m23);
        let d = vec3(m.m11, m.m12, m.m13).component_div(&c);
        let e = d + vec3(m.m31, m.m32, m.m33);
        let f = clamp_vec(&e, &vec3(0.0, 0.0, 0.0), &vec3(1.0, 1.0, 1.0));

        mix(
            &f,
            &vec3(1.0, 1.0, 1.0),
            smoothstep(1000.0, 0.0, self.temperature),
        )
    }
}

impl Shader for BlueLightFilter {
    fn name() -> &'static str {
        "Blue Light Filter"
    }

    fn reverse(&self, pixel: [u8; 3]) -> [u8; 3] {
        let mut color = vec3(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        );
        color = color.component_div(&self.color_temperature_to_rgb());

        [
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
        ]
    }
}
