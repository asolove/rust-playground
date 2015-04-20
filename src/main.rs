use std::thread::sleep_ms;
use std::io::stdout;
use std::io::Write;

fn main() {
    let prefix = "\x1B[";
    print!("{}32;1m[ ] ", prefix);
    print!("Hello, world!");
    stdout().flush();
    sleep_ms(500);
    print!("Stuff and things");
    stdout().flush();
    sleep_ms(500);
    print!("More stuff");
    stdout().flush();
    sleep_ms(400);
    println!("{}0G{}KMore stuff!", prefix, prefix);
}
