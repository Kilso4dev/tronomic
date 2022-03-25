use tokio::time::{self, Instant, Duration};
use parking_lot::Mutex;
use std::sync::Arc;

pub async fn output_send(freq: f64, freq_monitor: Arc<Mutex<f64>>) -> ! {
    let mut interval = time::interval(Duration::from_secs_f64(1./freq));
    let mut last = Instant::now();

    loop {
        interval.tick().await;
        let start = Instant::now();



        let calc_time = start.elapsed();
        let mut fmon = freq_monitor.lock();
        *fmon =  1e9/(start-last).as_nanos() as f64;

        last = start;
    }
}
