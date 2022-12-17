use std::io::Error;
use std::path::PathBuf;
use tokio::{fs, io};
use screenshots::Screen;

// pub async fn capture_screenshot(save_path: &PathBuf) -> Result<(), String>  {
//     if let Some(screens) = Screen::all() {
//         if screens.len() == 0 {
//             return Err(format!("Couldn't find main screen"));
//         }
//
//         match screens[0].capture() {
//             Some(image) => {
//                 let buffer = image.buffer();
//                 fs::write(&save_path, &buffer).await?;
//             }
//             None => return Err(format!("Couldn't capture screenshot")),
//         };
//     } else {
//         Err(format!("Couldn't retrieve screen"))
//     }
//
//     Ok(())
// }