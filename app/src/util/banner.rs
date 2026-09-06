pub fn print_banner() {
    const BANNER: &str = include_str!("banner.txt");
    const VERSION: &str = "0.1.0";
    const AUTHOR: &str = "Daniel Dias";

    const ORANGE: &str = "\x1b[38;5;208m";
    const DIM: &str = "\x1b[90m";
    const RESET: &str = "\x1b[0m";

    print!("{}{}{}", ORANGE, BANNER, RESET);
    println!("{} v{} — {} {}", DIM, VERSION, AUTHOR, RESET);
}
