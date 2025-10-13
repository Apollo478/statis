use x11rb::rust_connection::RustConnection;


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
