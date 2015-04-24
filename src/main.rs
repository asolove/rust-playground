extern crate term;

use std::io::prelude::*;
use std::thread::sleep_ms;

static PREFIX: &'static str = "\x1b[";

fn main() {
    status("Starting shit.", false);
    sleep_ms(500);
    status("Doing things", false);
    sleep_ms(500);
    status("Stuff is done, yo.", true);
    sleep_ms(500);
    for times in 0..10 {
        for i in 0..3 {
            spin(i);
            sleep_ms(128);
        }
    }
}

fn status(msg: &str, done: bool) {
    clear();
    let color = if done { term::color::GREEN } else { term::color::YELLOW };
    let status = if done { "\u{2713}" } else { " " };

    let mut t = term::stdout().unwrap();
    t.fg(color).unwrap();
    (write!(t, "[{}] {}\n", status, msg)).unwrap();
    t.reset().unwrap();
    (t.flush()).unwrap();
}

fn clear() {
    print!("{}F{}K", PREFIX, PREFIX);
}


fn spin(i: usize) {
    let states = vec!["-", "\\", "|", "/"];
    clear();
    println!(" [{}] Spinning", states[i]);
}
