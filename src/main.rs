use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid argument length");
        return;
    }

    println!("{:?}", args);
}
