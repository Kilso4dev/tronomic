use tokio::time::{self, Instant, Duration};
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;
use crate::dmx::DmxState;
use crate::integrations::artnet;

pub async fn output_send(freq: f64, tron_state: crate::TronomicState) -> ! {
    let mut interval = time::interval(Duration::from_secs_f64(1./freq));
    let mut last = Instant::now();
    let freq_mon = tron_state.fps_outp;
    let dmx_state = tron_state.dmx_state;
    let conns = tron_state.connections;

    //let artnet_con = artnet::ArtnetConnection::new();
    loop {
        interval.tick().await;
        let start = Instant::now();

        for c in &dmx_state.read().universes {
        }

        let calc_time = start.elapsed();
        *freq_mon.write() = 1e9/(start-last).as_nanos() as f64;

        last = start;
    }
}
