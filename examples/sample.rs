use tinyshader::*;

fn main() {
    let output = CompilerOptions::new()
        .stage(ShaderStage::Fragment)
        .source(include_str!("shader.hlsl"), None)
        .entry_point("main")
        .compile();

    match output {
        Ok(output) => {
            println!("Compilation successful! Output length: {}", output.len());
        },
        Err(error_str) => {
            print!("Compile error:\n {}", error_str);
        }
    }
}
