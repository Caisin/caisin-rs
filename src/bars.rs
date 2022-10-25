use std::{
    io::{self, BufWriter, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::{Duration, Instant},
};
pub struct Bar {
    arc_bol: Arc<AtomicBool>,
}
impl Bar {
    pub fn close(&mut self) {
        self.arc_bol.fetch_and(false, Ordering::Relaxed);
    }
}

impl Drop for Bar {
    fn drop(&mut self) {
        self.close()
    }
}

pub fn print_use_time(pre: &str) -> Bar {
    let now = Instant::now();
    let bar = Arc::new(AtomicBool::new(true));
    let arc = bar.clone();
    let pre = pre.to_string();
    tokio::spawn(async move {
        let mut sw = BufWriter::new(io::stdout());
        while arc.load(Ordering::Relaxed) {
            let str = format!("{pre}{:?}", now.elapsed());
            sw.write_fmt(format_args!("\r{str}")).unwrap();
            sw.flush().unwrap();
            sleep(Duration::from_millis(400));
        }
        let str = format!("{pre}{:?}", now.elapsed());
        sw.write_fmt(format_args!("\r{str}")).unwrap();
        sw.flush().unwrap();
    });
    Bar { arc_bol: bar }
}
