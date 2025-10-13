use image::{ImageBuffer, Rgb};
use x11rb::{connection::Connection, protocol::xproto::{get_image, ImageFormat}, rust_connection::RustConnection};


pub struct ScreenCapture {
    conn: RustConnection,
    screen_num: usize
}

pub struct CaptureRegion {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl ScreenCapture {
    pub fn new() -> Result<Self,Box<dyn std::error::Error>> {
        let (conn,screen_num) = x11rb::connect(None)?;
        Ok(Self { conn: (conn), screen_num: (screen_num) } ) 
    }
    
    pub fn screen_dimensions(&self) -> (u16,u16) {
        let screen = &self.conn.setup().roots[self.screen_num];
        (screen.width_in_pixels,screen.height_in_pixels)
    }

    pub fn capture_region(&self,region: CaptureRegion) -> Result<ImageBuffer<Rgb<u8>,Vec<u8>>,Box<dyn std::error::Error>> {
        let screen = &self.conn.setup().roots[self.screen_num];
        let image  =  get_image(&self.conn, ImageFormat::Z_PIXMAP,screen.root, region.x as i16, region.y as i16, region.width, region.height, u32::MAX).expect("Could not get image");
        let reply = image.reply()?;
        let rgb = bgrx_to_rgb(reply.data);

        ImageBuffer::from_raw(region.width as u32, region.height as u32, rgb).ok_or_else(|| "Invalid buffer size".into())
    }

    pub fn capture_full_screen(&self) -> Result<ImageBuffer<Rgb<u8>,Vec<u8>>,Box<dyn std::error::Error>> {
        let (width, height) = self.screen_dimensions();
        self.capture_region(CaptureRegion { x: (0), y: (0), width: (width), height: (height) })
    }

}




fn bgrx_to_rgb(data: Vec<u8>) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(data.len() / 4 * 3);
    for chunk in data.chunks_exact(4) {
        let b = chunk[0];
        let g = chunk[1];
        let r = chunk[2];
        rgb.push(r);
        rgb.push(g);
        rgb.push(b);
    }
    rgb
}
