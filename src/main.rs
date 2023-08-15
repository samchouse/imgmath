use image;
use nalgebra_glm::*;

fn main() {
    let image = image::open("./input.png").unwrap().to_rgba8();

    for mut pixel in image.pixels() {
        let mut color = vec3(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64);
        // color *= mix(
        //     &vec3(1.0, 1.0, 1.0),
        //     &vec3(1.0, 1.0, 1.0),
        //     // dot(
        //     //     color,
        //     //     vec3(0.2126, 0.7152, 0.0722)
        //     //         / max(dot(color, vec3(0.2126, 0.7152, 0.0722)), 1e-5),
        //     // ),
        //     1.0,
        // );

        color = mix(&color, &color, 1.0);

        let out = vec4(color[0], color[1], color[2], pixel[3] as f64);

        pixel[0] = out[0] as u8;
        pixel[1] = out[1] as u8;
        pixel[2] = out[2] as u8;
        pixel[3] = out[3] as u8;
    }
}

fn color_temp_to_rgb(temperature: f64) {
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

    nalgebra::Matrix1x3::<u64>::zeros() / nalgebra::Matrix3x1::<u64>::zeros();

    let a = clamp_scalar(temperature, 1000.0, 40000.0);
    let b = vec3(a, a, a);
    let c =
        mat3(m[0], m[0], m[0], m[0], m[0], m[0], m[0], m[0], m[0]) / (b + vec3(m[1], m[1], m[1]));
    let d = c + vec3(m[2], m[2], m[2]);
}
