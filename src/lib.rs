use crate::vulkan_init::{setup, example_triangle};

mod vulkan_init;

pub fn start() {
    let context = setup();
    example_triangle(context);
}

pub fn stop() {

}