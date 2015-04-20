use std::thread::sleep_ms;

static PREFIX: &'static str = "\x1b[";

fn main() {
    status("Waiting for things", false);
    sleep_ms(500);
    status("Doing things", false);
    sleep_ms(500);
    status("Stuff is done, yo.", true);
    sleep_ms(500);
}

fn status(msg: &str, done: bool) {
    clear();
    let color = if done { 32 } else { 33 };
    let status = if done { "\u{2713}" } else { " " };
    println!("{}{};1m[{}]{}0m {}", PREFIX, color, status, PREFIX, msg);
}

fn clear() {
    print!("{}F{}K", PREFIX, PREFIX);
}
