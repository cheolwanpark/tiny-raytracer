use wgpu;

pub struct Instance {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    pub(super) encoder: wgpu::CommandEncoder,
}

type Backends = wgpu::Backends;
type PowerPreference = wgpu::PowerPreference;
type Features = wgpu::Features;
type Limits = wgpu::Limits;
type MemoryHints = wgpu::MemoryHints;

pub struct InstanceOption {
    backends: Backends,
    power_preference: PowerPreference,
    features: Features,
    limits: Limits,
    memory_hints: MemoryHints,
}

impl Instance {
    pub async fn new(option: InstanceOption) -> Option<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: option.backends,
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        let adapter_option = wgpu::RequestAdapterOptions {
            power_preference: option.power_preference,
            force_fallback_adapter: false,
            compatible_surface: None,
        };
        let adapter = instance.request_adapter(&adapter_option).await?;

        let device_desc = wgpu::DeviceDescriptor {
            label: Some("tiny-raytracer-accel-device"),
            required_features: option.features,
            required_limits: option.limits,
            memory_hints: option.memory_hints,
        };
        let (device, queue) = adapter.request_device(&device_desc, None).await.ok()?;

        let encoder_desc = wgpu::CommandEncoderDescriptor {
            label: None,
        };
        let encoder = device.create_command_encoder(&encoder_desc);

        Some(Instance {
            instance,
            adapter,
            device,
            queue,
            encoder
        })
    }

    pub fn wait(&self) {
        self.device.poll(wgpu::Maintain::Wait);
    }

    pub fn poll(&self) -> bool {
        self.device.poll(wgpu::Maintain::Poll).is_queue_empty()
    }
}

impl InstanceOption {
    pub fn new() -> Self {
        InstanceOption {
            backends: Backends::default(),
            power_preference: PowerPreference::default(),
            features: Features::default(),
            limits: Limits::default(),
            memory_hints: MemoryHints::default(),
        }
    }

    pub fn backends(self, backends: Backends) -> Self {
        Self {
            backends,
            power_preference: self.power_preference,
            features: self.features,
            limits: self.limits,
            memory_hints: self.memory_hints
        }
    }

    pub fn power_preference(self, power_preference: PowerPreference) -> Self {
        Self {
            backends: self.backends,
            power_preference,
            features: self.features,
            limits: self.limits,
            memory_hints: self.memory_hints
        }
    }

    pub fn features(self, features: Features) -> Self {
        Self {
            backends: self.backends,
            power_preference: self.power_preference,
            features,
            limits: self.limits,
            memory_hints: self.memory_hints
        }
    }

    pub fn limits(self, limits: Limits) -> Self {
        Self {
            backends: self.backends,
            power_preference: self.power_preference,
            features: self.features,
            limits,
            memory_hints: self.memory_hints
        }
    }

    pub fn memory_hints(self, memory_hints: MemoryHints) -> Self {
        Self {
            backends: self.backends,
            power_preference: self.power_preference,
            features: self.features,
            limits: self.limits,
            memory_hints,
        }
    }
}