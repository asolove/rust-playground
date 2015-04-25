extern crate term;

use std::io::prelude::*;
use std::thread::sleep_ms;

fn main() {
    status("Starting shit.", false);
    println!("I'm an interrupting output!");
    sleep_ms(100);
    status("Doing things", false);
    sleep_ms(100);
    println!("I am also a console output that's unaware of animation stuff.");
    sleep_ms(100);
    for _times in 0..10 {
        for i in 0..3 {
            spin(i);
            sleep_ms(128);
        }
    }
    status("Stuff is done, yo.\n", true);
}

fn status(msg: &str, done: bool) {
    clear();
    let color = if done { term::color::GREEN } else { term::color::YELLOW };
    let status = if done { "\u{2713}" } else { " " };

    let mut t = term::stdout().unwrap();
    t.fg(color).unwrap();
    (write!(t, "[{}] {} ", status, msg)).unwrap();
    t.reset().unwrap();
    (t.flush()).unwrap();
}

fn clear() {
    let mut t = term::stdout().unwrap();
    t.delete_line().unwrap();
    print!("\r");
}


fn spin(i: usize) {
    let states = vec!["-", "\\", "|", "/"];
    clear();
    let mut t = term::stdout().unwrap();
    (write!(t, " [{}] Spinning ", states[i])).unwrap();
    (t.flush()).unwrap();
}
