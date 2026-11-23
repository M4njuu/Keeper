use crate::common::console::framebuffer::Framebuffer;

pub struct Gba{
    pub framebuffer: Framebuffer,
}

impl Gba{
    pub fn new() -> Self{
        Self {
            framebuffer: Framebuffer::new(240, 160),
        }
    }

    pub fn test(&self){
        println!("Hi! Im Gba!");
    }
}