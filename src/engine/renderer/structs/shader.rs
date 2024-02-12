use std::{fs, path::Path};

use ash::{
    vk::{PipelineShaderStageCreateInfo, ShaderModule, ShaderModuleCreateInfo, ShaderStageFlags},
    Device,
};

pub struct Shader {
    device: Device,
    stage: ShaderStageFlags,
    module: ShaderModule,
    pub stage_create_info: PipelineShaderStageCreateInfo,
}

impl Shader {
    pub fn from_path(device: &Device, path: &str) -> Result<Shader, String> {
        let file_ending = match Path::new(path).extension() {
            Some(file_ending) => file_ending.to_str().unwrap(),
            None => "vert",
        };

        let stage = match file_ending {
            "vert" => ShaderStageFlags::VERTEX,
            "frag" => ShaderStageFlags::FRAGMENT,
            _ => {
                return Err(format!("Unknown shader type: {}", file_ending));
            }
        };

        let shader_text = fs::read_to_string(path).expect("Failed to load vertex shader");

        let compiler = shaderc::Compiler::new().unwrap();
        let mut compiler_options = shaderc::CompileOptions::new().unwrap();

        compiler_options.add_macro_definition("EP", Some("main"));

        let spirv_binary_data = compiler
            .compile_into_spirv(
                &shader_text,
                shaderc::ShaderKind::Vertex,
                "text.vert",
                "main",
                Some(&compiler_options),
            )
            .unwrap();

        let shader_module_create_info = ShaderModuleCreateInfo::builder()
            .code(spirv_binary_data.as_binary())
            .build();

        let module = match unsafe { device.create_shader_module(&shader_module_create_info, None) }
        {
            Ok(shader_module) => shader_module,
            Err(err) => {
                return Err(format!("Failed to create shader module: {}", err));
            }
        };

        let stage_create_info = PipelineShaderStageCreateInfo::builder()
            .stage(stage)
            .module(module)
            .name(&std::ffi::CString::new("main").unwrap())
            .build();

        Ok(Shader {
            device: device.clone(),
            stage,
            module,
            stage_create_info,
        })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_shader_module(self.module, None);
        }
    }
}
