use reqwest;
use reqwest::blocking;
use std::thread;
use std::time::Instant;

const REQ_COUNT: u32 = 1000;

fn main() {
    (0..10).for_each(|_| {
        thread::spawn(move || {
            let mut ok = 0;
            let start_time = Instant::now();
            for _ in 0..REQ_COUNT / 10 {
                if let Ok(_) = blocking::get("http://localhost:8000") {
                    ok += 1
                }
            }
            println!(
                "ok: {}, err: {} with time: {}",
                ok,
                REQ_COUNT / 10 - ok,
                start_time.elapsed().as_millis()
            );
        });
    });
    thread::sleep(std::time::Duration::from_secs(100));
}
