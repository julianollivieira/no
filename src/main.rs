mod attr;
mod helpers;
mod input;
mod screen;

use attr::IntoRawMode;
use input::async_stdin;
use std::io::{self, Read, Write};

/// The config for the editor.
pub struct EditorConfig {
    height: u16,
    width: u16,
    cx: u16,
    cy: u16,
}

fn render_frame(
    render_buf: &mut Vec<u8>,
    stdout: &mut impl io::Write,
    config: &EditorConfig,
) -> io::Result<()> {
    screen::hide_cursor(render_buf)?;
    screen::move_cursor_to_origin(render_buf)?;

    // render frame
    for y in 0..config.height {
        if y == config.height / 3 {
            // render welcome message
            let mut msg = format!("no editor -- version {}", env!("CARGO_PKG_VERSION"));
            if msg.len() > config.width.into() {
                msg.truncate(config.width.into());
            }

            // center welcome message
            let mut padding = (config.width as usize - msg.len()) / 2;
            if padding > 0 {
                render_buf.write(b"~")?;
                padding -= 1;
            }

            while padding > 0 {
                render_buf.write(b" ")?;
                padding -= 1;
            }

            render_buf.write(msg.as_bytes())?;
        } else {
            render_buf.write(b"~")?;
        }

        screen::clear_line(render_buf)?;
        if y < config.height - 1 {
            render_buf.write(b"\r\n")?;
        }
    }

    // render cursor
    // screen::move_cursor_to_origin(render_buf)?;
    screen::move_cursor_to_(render_buf, config.cy, config.cx)?;
    screen::show_cursor(render_buf)?;

    screen::render(stdout, render_buf)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdin = async_stdin();
    let mut stdout = io::stdout()
        .lock()
        .try_into_raw_mode()
        .expect("failed to enable raw mode for stdout");

    let (height, width) = screen::get_terminal_size()?;
    let config = EditorConfig {
        height,
        width,
        cx: 5,
        cy: 5,
    };

    let mut render_buf = Vec::new();

    loop {
        render_frame(&mut render_buf, &mut stdout.output, &config)?;

        // read input
        let mut input_buf = [0; 1];
        let bytes_read = stdin.read(&mut input_buf);

        if bytes_read.is_ok() {
            if input_buf[0] == 0x03 {
                break;
            }
        }
    }

    Ok(())
}
