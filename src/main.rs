// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use rand::Rng;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_generate({
        move |min: slint::SharedString, max: slint::SharedString| {
            let minimum: u32 = match min.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    if let Some(strong_ui) = ui_handle.upgrade() {
                        strong_ui.set_output("Invalid min".into());
                    }
                    return;
                }
            };
            let maximum: u32 = match max.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    if let Some(strong_ui) = ui_handle.upgrade() {
                        strong_ui.set_output("Invalid max".into());
                    }
                    return;
                }
            };
            if minimum >= maximum {
                if let Some(strong_ui) = ui_handle.upgrade() {
                    strong_ui.set_output("Min > Max".into());
                }
                return;
            }
            let mut rng = rand::rng();
            let num: u32 = rng.random_range(minimum..maximum);
            if let Some(strong_ui) = ui_handle.upgrade() {
                strong_ui.set_output(num.to_string().into());
            }
        }
    });

    ui.run()?;
    Ok(())
}
