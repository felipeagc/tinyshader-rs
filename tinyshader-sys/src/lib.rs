use std::os::raw::c_char;

#[repr(C)]
pub struct TsCompilerOptions {
    _private: [u8; 0]
}

#[repr(C)]
pub struct TsCompilerOutput {
    _private: [u8; 0]
}

#[repr(C)]
pub enum TsShaderStage {
    Vertex = 0,
    Fragment = 1,
    Compute = 2,
}

extern "C" {
    pub fn tsCompilerOptionsCreate() -> *mut TsCompilerOptions;
    pub fn tsCompilerOptionsSetStage(options: *mut TsCompilerOptions, stage: TsShaderStage);
    pub fn tsCompilerOptionsSetEntryPoint(options: *mut TsCompilerOptions, entry_point: *const c_char, length: usize);
    pub fn tsCompilerOptionsSetSource(
        options: *mut TsCompilerOptions,
        source: *const c_char,
        source_length: usize,
        path: *const c_char,
        path_length: usize,
    );
    pub fn tsCompilerOptionsAddIncludePath(options: *mut TsCompilerOptions, include_path: *const c_char, length: usize);
    pub fn tsCompilerOptionsDestroy(options: *mut TsCompilerOptions);

    pub fn tsCompile(options: *mut TsCompilerOptions) -> *mut TsCompilerOutput;
    pub fn tsCompilerOutputGetErrors(output: *mut TsCompilerOutput) -> *const c_char;
    pub fn tsCompilerOutputGetSpirv(output: *mut TsCompilerOutput, length: *mut usize) -> *const u8;
    pub fn tsCompilerOutputDestroy(output: *mut TsCompilerOutput);
}


