extern crate cc;

fn main() {
    cc::Build::new()
        .file("tinyshader/tinyshader/tinyshader_unity.c")
        .compile("libtinyshader.a");
}
