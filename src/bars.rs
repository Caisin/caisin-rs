use std::{
    io::{self, BufWriter, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use tokio::time::sleep;

use crate::defer::Defer;
pub struct Bar {
    switch: Arc<AtomicBool>,
}
impl Bar {
    pub fn close(&mut self) {
        self.switch.fetch_and(false, Ordering::Relaxed);
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
    let dlab=pre.clone();
    tokio::spawn(async move {
        let _defer = Defer(move ||{
            let mut sw = BufWriter::new(io::stdout());
            let str = format!("{dlab}{:?}", now.elapsed());
            sw.write_fmt(format_args!("\r{str}==\n")).unwrap();
            sw.flush().unwrap();
        });

        let mut sw = BufWriter::new(io::stdout());
        while arc.load(Ordering::Relaxed) {
            let str = format!("{pre}{:?}", now.elapsed());
            sw.write_fmt(format_args!("\r{str}")).unwrap();
            sw.flush().unwrap();
            sleep(Duration::from_millis(400)).await;
        }
      
    });
    Bar { switch: bar }
}
