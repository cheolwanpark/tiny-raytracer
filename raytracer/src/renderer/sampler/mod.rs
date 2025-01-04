use crate::hittable::world::World;
use super::{imager::SampledColor, pointgen::SamplePoint};
use flume::{Receiver, Sender};

mod cpu;
pub use cpu::CpuSampler;

pub mod metal;

pub trait Sampler {
    fn sampling(
        self, 
        world: &World,
        in_channel: Receiver<SamplePoint>,
        out_channel: Sender<SampledColor>,
    ) -> impl std::future::Future<Output = ()> + Send;
}

