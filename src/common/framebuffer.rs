///画面のデータ
pub struct Framebuffer{
    width:u16,          // 画面の横のドット数
    height:u16,         // 画面の縦のドット数
    pixels:Vec<u32>,    // ドットの色の配列(解像度は可変)
}

impl Framebuffer{
    pub fn new(width:u16, height:u16) -> Self{
        let total_pixels: usize = (width * height) as usize;

        Self {
            width,
            height,
            pixels: vec![0; total_pixels],
        }
    }

    pub fn test(&self){
        println!("Hi! Im framebuffer!!");
    }
}