use std::{thread, time::Duration};

use ::concurrency::AmapMetrics;
use anyhow::Result;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let x = &[
        "request.page.0",
        "request.page.1",
        "request.page.2",
        "request.page.3",
        "request.page.4",
        "call.thread.worker.0",
        "call.thread.worker.1",
    ];
    let metrics = AmapMetrics::new(x);
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

fn task_worker(idx: usize, metrics: AmapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rand::Rng::gen_range(
            &mut rng,
            100..4300,
        )));
        _ = metrics.inc(format!("call.thread.worker.{}", idx));
    });
}

fn request_worker(metrics: AmapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rand::Rng::gen_range(
            &mut rng,
            50..900,
        )));
        let page = rand::Rng::gen_range(&mut rng, 0..5);
        _ = metrics.inc(format!("request.page.{}", page));
    });
}
