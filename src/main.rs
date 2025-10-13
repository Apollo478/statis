use crate::capture::ScreenCapture;

mod capture;
fn main() {
    let capture = ScreenCapture::new().expect("failed to get new capture");
    let fs = capture.capture_full_screen().expect("Could not capture screen");
    fs.save("ss.png");
}
