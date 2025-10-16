mod capture;
mod statis_ui;

use statis_ui::X11ScreenshotTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tool = X11ScreenshotTool::new()?;
    tool.run()?;
    Ok(())
}
