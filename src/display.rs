fn color_code(color: u8) -> &'static str {
    match color {
        0 => "\x1b[1;31m", // Red
        1 => "\x1b[1;32m", // Green
        2 => "\x1b[1;33m", // Yellow
        3 => "\x1b[1;34m", // Blue
        4 => "\x1b[1;35m", // Magenta
        5 => "\x1b[1;36m", // Cyan
        6 => "\x1b[1;37m", // White
        _ => "\x1b[0m",    // Reset
    }
}

pub fn print_header(text: &str) {
    let c = '=';
    let title_size = text.len() + 2;
    let sz = 80;
    let left_size = (sz - title_size) / 2;
    let right_size = sz - title_size - left_size;
    let color_code: &str = color_code(3);
    println!("\n{}{}", color_code, c.to_string().repeat(sz));
    println!("{}{}{}", c.to_string(), " ".repeat(sz - 2), c.to_string());
    println!("{}{}{}{}{}", c.to_string(), " ".repeat(left_size), text, " ".repeat(right_size), c.to_string());
    println!("{}{}{}", c.to_string(), " ".repeat(sz - 2), c.to_string());
    println!("{}{}\n", c.to_string().repeat(sz), "\x1b[0m");
}

pub fn print_title(text: &str) {
    let c = '¤';
    let title_size = text.len() + 2;
    let sz = 80;
    let left_size = (sz - title_size) / 2;
    let right_size = sz - title_size - left_size;
    let color_code: &str = color_code(3);
    println!("\n{}{}", color_code, c.to_string().repeat(sz));
    println!("{}{}{}{}{}{}", color_code, c.to_string().repeat(left_size), " ", text, " ", c.to_string().repeat(right_size));
    println!("{}{}\n", c.to_string().repeat(sz), "\x1b[0m");
}

pub fn print_sep() {
    let c = '~';
    let count = 80;
    let color_code: &str = color_code(2);
    println!("\n{}{}{}\n", color_code, c.to_string().repeat(count), "\x1b[0m");
}

