use std::{thread, time::Duration};

use ::concurrency::CMetrics;
use anyhow::Result;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = CMetrics::new();
    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }
    for _ in 0..M {
        request_worker(metrics.clone());
    }
    loop {
        thread::sleep(Duration::from_secs(3));
        println!("{}", metrics);
    }
    #[allow(unreachable_code)]
    Ok(())
}

fn task_worker(idx: usize, metrics: CMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rand::Rng::gen_range(
            &mut rng,
            100..4300,
        )));
        _ = metrics.inc(format!("call.thread.worker.{}", idx));
    });
}

fn request_worker(metrics: CMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rand::Rng::gen_range(
            &mut rng,
            50..900,
        )));
        let page = rng.gen_range(1..256);
        _ = metrics.inc(format!("request.page.{}", page));
    });
}
