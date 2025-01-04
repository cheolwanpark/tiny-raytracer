pub enum MetalGeometryMasks {
    Quad = 1,
    Sphere = 2,
}

impl From<MetalGeometryMasks> for u32 {
    fn from(value: MetalGeometryMasks) -> Self {
        value as u32
    }
}