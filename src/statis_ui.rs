use x11rb::{connection::Connection, protocol::Event};
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::COPY_DEPTH_FROM_PARENT;
use image::{ImageBuffer, Rgb};
use std::error::Error;

use crate::capture::{ScreenCapture, CaptureRegion};

pub struct X11ScreenshotTool {
    conn: RustConnection,
    screen_num: usize,
    window: u32,
    root: u32,
    width: u16,
    height: u16,
    selecting: bool,
    start_x: i16,
    start_y: i16,
    current_x: i16,
    current_y: i16,
    screenshot_pixmap: u32, 
    gc: u32,
}

impl X11ScreenshotTool {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (conn, screen_num) = x11rb::connect(None)?;
        let screen = &conn.setup().roots[screen_num];
        let root = screen.root;
        let width = screen.width_in_pixels;
        let height = screen.height_in_pixels;

        let capture = ScreenCapture::new()?;
        let screenshot = capture.capture_full_screen()?;

        let window = conn.generate_id()?;
        
        let win_aux = CreateWindowAux::new()
            .background_pixel(0)
            .border_pixel(0)
            .override_redirect(1)
            .event_mask(
                EventMask::EXPOSURE
                    | EventMask::BUTTON_PRESS
                    | EventMask::BUTTON_RELEASE
                    | EventMask::POINTER_MOTION
                    | EventMask::KEY_PRESS
            );

        conn.create_window(
            COPY_DEPTH_FROM_PARENT,
            window,
            root,
            0,
            0,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            screen.root_visual,
            &win_aux,
        )?;

        let screenshot_pixmap = conn.generate_id()?;
        conn.create_pixmap(24, screenshot_pixmap, window, width, height)?;
        
        let gc = conn.generate_id()?;
        conn.create_gc(gc, screenshot_pixmap, &CreateGCAux::new())?;
        
        let mut data = Vec::with_capacity((width as usize) * (height as usize) * 4);
        for pixel in screenshot.pixels() {
            data.push(pixel[2]); 
            data.push(pixel[1]); 
            data.push(pixel[0]); 
            data.push(0);        
        }
        
        conn.put_image(
            ImageFormat::Z_PIXMAP,
            screenshot_pixmap,
            gc,
            width,
            height,
            0,
            0,
            0,
            24,
            &data,
        )?;

        // Map window
        conn.map_window(window)?;
        conn.configure_window(
            window,
            &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
        )?;

        conn.flush()?;

        Ok(Self {
            conn,
            screen_num,
            window,
            root,
            width,
            height,
            selecting: false,
            start_x: 0,
            start_y: 0,
            current_x: 0,
            current_y: 0,
            screenshot_pixmap,
            gc,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // Initial draw
        self.redraw()?;
        
        loop {
            let event = self.conn.wait_for_event()?;
            
            match event {
                Event::Expose(_) => {
                    self.redraw()?;
                }
                Event::ButtonPress(event) => {
                    if event.detail == 1 {
                        self.selecting = true;
                        self.start_x = event.event_x;
                        self.start_y = event.event_y;
                        self.current_x = event.event_x;
                        self.current_y = event.event_y;
                        self.redraw()?;
                    }
                }
                Event::MotionNotify(event) => {
                    if self.selecting {
                        self.current_x = event.event_x;
                        self.current_y = event.event_y;
                        self.redraw()?;
                    }
                }
                Event::ButtonRelease(event) => {
                    if event.detail == 1 && self.selecting {
                        self.selecting = false;
                        
                        let x1 = self.start_x.min(self.current_x);
                        let y1 = self.start_y.min(self.current_y);
                        let x2 = self.start_x.max(self.current_x);
                        let y2 = self.start_y.max(self.current_y);
                        
                        let width = (x2 - x1) as u16;
                        let height = (y2 - y1) as u16;
                        
                        if width > 0 && height > 0 {
                            self.capture_selection(x1, y1, width, height)?;
                        }
                        
                        return Ok(());
                    }
                }
                Event::KeyPress(event) => {
                    if event.detail == 9 {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
    }

    fn redraw(&self) -> Result<(), Box<dyn Error>> {
        // Copy screenshot from pixmap to window (very fast)
        self.conn.copy_area(
            self.screenshot_pixmap,
            self.window,
            self.gc,
            0, 0,
            0, 0,
            self.width,
            self.height,
        )?;
        
        // Draw selection rectangle if selecting
        if self.selecting {
            let x = self.start_x.min(self.current_x);
            let y = self.start_y.min(self.current_y);
            let width = (self.start_x.max(self.current_x) - x) as u16;
            let height = (self.start_y.max(self.current_y) - y) as u16;

            // Update GC for drawing rectangle
            self.conn.change_gc(
                self.gc,
                &ChangeGCAux::new()
                    .foreground(0x4099FF)
                    .line_width(3)
            )?;

            self.conn.poly_rectangle(
                self.window,
                self.gc,
                &[Rectangle { x, y, width, height }],
            )?;
        }
        
        self.conn.flush()?;
        Ok(())
    }

    fn capture_selection(&self, x: i16, y: i16, width: u16, height: u16) -> Result<(), Box<dyn Error>> {
        println!("Capturing region: x={}, y={}, width={}, height={}", x, y, width, height);
        
        let capture = ScreenCapture::new()?;
        let img = capture.capture_region(CaptureRegion {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
            width,
            height,
        })?;
        
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let path = format!("{}/Downloads/screenshot.png", home);
        img.save(&path)?;
        
        println!("Saved to {}", path);
        
        Ok(())
    }
}

impl Drop for X11ScreenshotTool {
    fn drop(&mut self) {
        let _ = self.conn.free_gc(self.gc);
        let _ = self.conn.free_pixmap(self.screenshot_pixmap);
        let _ = self.conn.destroy_window(self.window);
        let _ = self.conn.flush();
    }
}
