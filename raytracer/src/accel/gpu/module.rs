use std::{borrow::Cow, fs::File, io::Read, ops::Deref, path::Path};

use wgpu;

use super::instance::Instance;

pub struct Module(wgpu::ShaderModule);

pub struct ModuleSourceBuilder {
    source: String,
}

impl Module {
    fn new(instance: &Instance, source: &str) -> Self {
        let desc = wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source))
        };
        let module = instance.device.create_shader_module(desc);
        Self(module)
    }

    fn new_with_label(instance: &Instance, label: &str, source: &str) -> Self {
        let desc = wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source))
        };
        let module = instance.device.create_shader_module(desc);
        Self(module)
    }
}

impl Deref for Module {
    type Target = wgpu::ShaderModule;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Instance {
    pub fn create_module(&self, source: &str) -> Module {
        Module::new(self, source)
    }

    pub fn create_module_with_label(&self, label: &str, source: &str) -> Module {
        Module::new_with_label(self, label, source)
    }
}

impl ModuleSourceBuilder {
    pub fn new() -> Self {
        Self { source: String::new() }
    }

    pub fn add(&mut self, source: &str) {
        self.source.push_str(source)
    }

    pub fn add_file(&mut self, path: &Path) {
        let mut file = File::open(path).expect(&format!("failed to open {}", path.to_str().unwrap()));
        let mut source = String::new();
        file.read_to_string(&mut source).expect(&format!("failed to read {}", path.to_str().unwrap()));
        self.add(&source);
    }
}

impl Deref for ModuleSourceBuilder {
    type Target = str;
    fn deref(&self) -> &str {
        &self.source
    }
}