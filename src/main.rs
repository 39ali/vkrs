use std::sync::Arc;

use pretty_bytes::converter::convert;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::device::{Device, QueuesIter};
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::QueueFamily;
fn main() {
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    println!("{:?}:{:?}", physical.ty(), physical.name());
    print_device_mems(&physical);
    let queue_family = init_queue_families(&physical);
    let (_device, mut queues) = init_device(&physical, &queue_family);
    let _queue = queues.next().unwrap();
}

fn print_device_mems(physical: &PhysicalDevice) {
    for mem in physical.memory_heaps() {
        println!(
            "id:{:} , size:{:} , device Local:{:}",
            mem.id(),
            convert(mem.size() as f64),
            mem.is_device_local()
        )
    }
}

fn init_queue_families<'a>(physical: &'a PhysicalDevice) -> QueueFamily<'a> {
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    return queue_family;
}

fn init_device(physical: &PhysicalDevice, queue_family: &QueueFamily) -> (Arc<Device>, QueuesIter) {
    let (device, queues) = {
        Device::new(
            *physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(*queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };

    return (device, queues);
}
