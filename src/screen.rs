/// Clears the screen and moves the cursor to the top left corner.
pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn draw() {
    for _ in 0..24 {
        print!("~\r\n");
    }
}
