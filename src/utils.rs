use std::thread::sleep;
use std::time::Duration;

pub fn wait_for<F>(f: F)
where
    F: Fn() -> bool,
{
    let mut done = false;
    while !done {
        done = f();
        sleep(Duration::from_secs_f32(0.05));
    }
}

pub fn abort(message: &str) {
    eprintln!("{}", message);
    std::process::exit(1);
}
