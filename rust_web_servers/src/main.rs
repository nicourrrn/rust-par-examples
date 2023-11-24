use rust_web_servers::{mutli_threaded_server, single_server};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() == 3 {
        let threading = args[1].parse::<u8>().unwrap();
        let duration = args[2].parse::<u8>().unwrap() == 1;
        println!("threading: {}, duration: {}", threading, duration);
        if threading == 1 {
            println!("Running single threaded server");
            single_server::run(duration);
        } else {
            println!("Running multi threaded server with {} threads", threading);
            mutli_threaded_server::run(threading as usize, duration);
        }
    }
}
