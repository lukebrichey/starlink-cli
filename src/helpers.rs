use termion::color;

// Helper to print colored output
pub fn print_colored<T: std::fmt::Debug>(msg: &str, value: &Option<T>) {
    match value {
        Some(v) => println!(
            "{}: {}{:?}{}", 
            msg,
            color::Fg(color::Red), 
            v,
            color::Fg(color::Reset)
        ),
        None => println!(
            "{}: {}None{}",
            msg,
            color::Fg(color::Green),
            color::Fg(color::Reset)
        ),
    }    
}

// Helper to print colored output based on percentage
pub fn print_color_percentage(percentage: f32) {
    if percentage >= 0.0 && percentage < 6.0 {
        println!("{}{}{}", color::Fg(color::Green), percentage, color::Fg(color::Reset))
    } else if percentage >= 6.0 && percentage < 11.0 {
        println!("{}{}{}", color::Fg(color::Yellow), percentage, color::Fg(color::Reset))
    } else {
        println!("{}{}{}", color::Fg(color::Red), percentage, color::Fg(color::Reset))
    }
}