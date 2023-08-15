use image;
use nalgebra_glm::*;

fn main() {
    let mut img = image::open("./input.png").unwrap().to_rgba8();

    img.pixels_mut().for_each(|pixel| {
        let mut color = vec3(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64);

        let a = mat3(
            color[0], color[1], color[2], color[0], color[1], color[2], color[0], color[1],
            color[2],
        ) * color_temp_to_rgb(4000.0);
        color = mix(&color, &a, 1.0);

        let out = vec4(color[0], color[1], color[2], pixel[3] as f64);

        pixel.0 = image::Rgba([out[0] as u8, out[0] as u8, out[0] as u8, out[0] as u8]).0;
    });

    img.save("out.png").unwrap();
}

fn color_temp_to_rgb(temperature: f64) -> TVec3<f64> {
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

    let a = clamp_scalar(temperature, 1000.0, 40000.0);
    let b = vec3(a, a, a);
    let c = b + vec3(m[1], m[1], m[1]);
    let d = vec3(m[0], m[0], m[0]).component_div(&c);
    let e = d + vec3(m[2], m[2], m[2]);
    let f = clamp_vec(&e, &vec3(0.0, 0.0, 0.0), &vec3(1.0, 1.0, 1.0));

    mix(
        &f,
        &vec3(1.0, 1.0, 1.0),
        smoothstep(1000.0, 0.0, temperature),
    )
}
