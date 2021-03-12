mod util;

use util::*;

use clipboard::{ClipboardContext, ClipboardProvider};
use std::{thread, time::Duration};
use web_view::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let page_content;
    if cfg!(debug_assertions) {
        page_content = Content::Url("http://localhost:3000");
    } else {
        page_content = Content::Html(include_str!("ui/dist/index.html"));
    }

    let scale_factor = {
        use winit::{event_loop::EventLoop, window::WindowBuilder};

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_maximized(false)
            .with_visible(false)
            .build(&event_loop)?;

        let scale_factor = window.scale_factor();
        println!("Scale factor: {}", scale_factor);

        drop(window);

        scale_factor
    };

    let mut webview = web_view::builder()
        .title("Emoji Picker")
        .content(page_content)
        .size(535, 665)
        .resizable(true)
        .frameless(false)
        .user_data(())
        .invoke_handler(|webview, arg| {
            if arg.contains("COPY_EMOJI") {
                let mut clipboard_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                clipboard_ctx
                    .set_contents(arg.replace("COPY_EMOJI ", ""))
                    .unwrap();

                webview.exit();

                thread::sleep(Duration::from_millis(10));

                let paste_cmd = Shell::cmd(String::from("xdotool key ctrl+v"));
                println!("{}", paste_cmd);
            }

            Ok(())
        })
        .debug(true)
        .build()?;

    webview.set_zoom_level(scale_factor);

    webview.run()?;

    Ok(())
}
