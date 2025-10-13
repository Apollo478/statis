mod capture;
use image::{ImageBuffer, Rgb};
use x11rb::{connection::Connection, protocol::xproto::{get_image, ImageFormat}};
fn main() {
    let (conn,screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let height = screen.height_in_pixels as u64;
    let full_width = screen.width_in_pixels as u64;
    let capture_x = 1920;
    let capture_width = (screen.width_in_pixels /2) as u64;
    let image  =  get_image(&conn, ImageFormat::Z_PIXMAP,screen.root, capture_x, 0, capture_width as u16, screen.height_in_pixels, u32::MAX).expect("Could not get image");
    let reply = image.reply().expect("could not get reply");
    let reply_size = reply.data.len();
    let pix_format = ((screen.root_depth + 8) / 8) as u64;
    let image_size = (capture_width * height * pix_format)  as usize;
    assert_eq!(reply_size,image_size);
    let rgb = bgrx_to_rgb(reply.data);

    let img = ImageBuffer::<Rgb<u8>,_>::from_raw(capture_width as u32, height as u32, rgb).expect("invalid buffer size");

    img.save("ss.png").unwrap();

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
