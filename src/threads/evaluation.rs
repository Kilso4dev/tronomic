use tokio::time::{self, Instant, Duration};
use parking_lot::Mutex;
use std::sync::Arc;

pub async fn process_eval(freq: f64, tron_state: crate::TronomicState) -> ! {
    let mut interval = time::interval(Duration::from_secs_f64(1./freq));
    let mut last = Instant::now();
    let freq_mon = tron_state.fps_eval;
    let graph = tron_state.graph;
    let frame = tron_state.frame;
    let time = tron_state.time;

    loop {
        interval.tick().await;
        let start = Instant::now();

        let calc_time = start.elapsed();
        *freq_mon.write() =  1e9/(start-last).as_nanos() as f64;

        last = start;
    }
}
