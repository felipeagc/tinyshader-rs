extern crate tinyshader_sys;

use std::ffi::{CStr, CString};
use tinyshader_sys::*;

pub use tinyshader_sys::TsShaderStage as ShaderStage;

pub struct CompilerOptions {
    options: *mut TsCompilerOptions,
}

impl CompilerOptions {
    pub fn new() -> Self {
        unsafe {
            Self {
                options: tsCompilerOptionsCreate(),
            }
        }
    }

    pub fn stage<'a>(&'a mut self, stage: ShaderStage) -> &'a mut Self {
        unsafe {
            tsCompilerOptionsSetStage(self.options, stage);
        }
        self
    }

    pub fn entry_point<'a>(&'a mut self, entry_point: &str) -> &'a mut Self {
        unsafe {
            let c_entry_point = CString::new(entry_point).unwrap();
            tsCompilerOptionsSetEntryPoint(
                self.options,
                c_entry_point.as_ptr(),
                c_entry_point.as_bytes().len(),
            );
        }
        self
    }

    pub fn source<'a>(&'a mut self, source: &str, path_opt: Option<&str>) -> &'a mut Self {
        unsafe {
            let c_source = CString::new(source).unwrap();

            if let Some(path) = path_opt {
                let c_path = CString::new(path).unwrap();
                tsCompilerOptionsSetSource(
                    self.options,
                    c_source.as_ptr(),
                    c_source.as_bytes().len(),
                    c_path.as_ptr(),
                    c_path.as_bytes().len(),
                );
            } else {
                tsCompilerOptionsSetSource(
                    self.options,
                    c_source.as_ptr(),
                    c_source.as_bytes().len(),
                    std::ptr::null(),
                    0,
                );
            }
        }
        self
    }

    pub fn include_path<'a>(&'a mut self, path: &str) -> &'a mut Self {
        unsafe {
            let c_path = CString::new(path).unwrap();
            tsCompilerOptionsAddIncludePath(
                self.options,
                c_path.as_ptr(),
                c_path.as_bytes().len(),
            );
        }
        self
    }

    pub fn compile(&self) -> Result<Vec<u8>, String> {
        unsafe {
            let output = tsCompile(self.options);
            let errors = tsCompilerOutputGetErrors(output);
            if !errors.is_null() {
                let c_errors = CStr::from_ptr(errors);
                let error_str = c_errors.to_str().unwrap().to_owned();
                tsCompilerOutputDestroy(output);
                return Err(error_str);
            }

            let mut spirv_size: usize = 0;
            let spirv_ptr: *const u8 = tsCompilerOutputGetSpirv(output, &mut spirv_size);

            let mut vec = Vec::<u8>::new();
            vec.reserve(spirv_size);
            vec.set_len(spirv_size);

            std::ptr::copy_nonoverlapping(spirv_ptr, vec.as_mut_ptr(), spirv_size);

            tsCompilerOutputDestroy(output);
            return Ok(vec);
        }
    }
}

impl Drop for CompilerOptions {
    fn drop(&mut self) {
        unsafe {
            tsCompilerOptionsDestroy(self.options);
        }
    }
}
