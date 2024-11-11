use anyhow::anyhow;
use anyhow::Result;
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: usize,
    v: usize,
}

#[allow(clippy::needless_borrows_for_generic_args)]
impl Msg {
    fn new(id: usize, valuex: usize) -> Self {
        Msg { id, v: valuex }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
    });
    consumer
        .join()
        .map_err(|e| anyhow!("Thread Join error: {:?}", e))?;

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value: usize = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        thread::sleep(Duration::from_millis(100));
    }
}
