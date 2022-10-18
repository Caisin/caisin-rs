use std::{
    io::{self, BufWriter, Write},
    thread::sleep,
    time::{Duration, Instant},
};

pub fn print_use_time() {
    let now = Instant::now();
    tokio::spawn(async move {
        let mut sw = BufWriter::new(io::stdout());
        loop {
            let str = format!("耗时:{:?}", now.elapsed());
            sw.write_fmt(format_args!("\r{str}")).unwrap();
            sw.flush().unwrap();
            sleep(Duration::from_millis(400));
        }
    });
}
