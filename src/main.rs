use image::{self, Rgba};
use nalgebra_glm::*;

fn main() {
    let mut img = image::open("./input.png").unwrap().to_rgba8();

    img.pixels_mut().for_each(|pixel| {
        *pixel = glsl_main(*pixel);
    });

    img.save("out.png").unwrap();
}

fn glsl_main(pixel: Rgba<u8>) -> Rgba<u8> {
    let mut color = vec3(
        pixel[0] as f64 / 255.0,
        pixel[1] as f64 / 255.0,
        pixel[2] as f64 / 255.0,
    );

    let a = color.component_div(&color_temp_to_rgb(4000.0));
    // color = mix(&color, &a, 1.0);

    // let colora = mix(&a, &color, 1.0);
    // colora.

    // // println!();
    // // println!("{:?}", a);
    // // println!("{:?}", colora);
    // // println!("{:?}", colorb);
    // // println!();

    // if a != colora {
    //     println!("a != colora");
    // };


    let out = vec4(
        color.x * 255.0,
        color.y * 255.0,
        color.z * 255.0,
        pixel[3] as f64,
    );

    image::Rgba([out.x as u8, out.y as u8, out.z as u8, 255])
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
    let c = b + vec3(m.m21, m.m22, m.m23);
    let d = vec3(m.m11, m.m12, m.m13).component_div(&c);
    let e = d + vec3(m.m31, m.m32, m.m33);
    let f = clamp_vec(&e, &vec3(0.0, 0.0, 0.0), &vec3(1.0, 1.0, 1.0));

    mix(
        &f,
        &vec3(1.0, 1.0, 1.0),
        smoothstep(1000.0, 0.0, temperature),
    )
}
