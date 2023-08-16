pub mod shaders;

#[cfg(feature = "headers")] // c.f. the `Cargo.toml` section
pub fn generate_headers() -> ::std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("imgmath.h")?
        .generate()
}
