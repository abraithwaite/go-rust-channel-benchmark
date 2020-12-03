#![feature(test)]
extern crate test;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    channel(10).await;
}

pub async fn channel(num_threads: u64) {
    let (tx, mut rx) = mpsc::channel(10000);

    for i in 0u64..num_threads {
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(i).await.unwrap();
        });
    }

    for _ in 0u64..num_threads {
        let x = rx.recv().await.unwrap();
    }
}

pub async fn channel_threads(num_threads: u64) {
    let (tx, mut rx) = mpsc::channel(10000);

    for i in 0u64..num_threads {
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(i).await.unwrap();
        });
    }

    for _ in 0u64..num_threads {
        let _ = rx.recv().await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use tokio::runtime::Runtime;


    #[bench]
    fn bench_rust_channel_10_u64(b: &mut Bencher) {
        let rt = Runtime::new().unwrap();
        b.iter(|| rt.block_on(channel(10)));
    }

    #[bench]
    fn bench_rust_channel_10000_u64(b: &mut Bencher) {
        let rt = Runtime::new().unwrap();
        b.iter(|| rt.block_on(channel(10000)));
    }

    #[bench]
    fn bench_rust_channel_threads_10_u64(b: &mut Bencher) {
        let rt = Runtime::new().unwrap();
        b.iter(|| rt.block_on(channel_threads(10)));
    }

    #[bench]
    fn bench_rust_channel_threads_10000_u64(b: &mut Bencher) {
        let rt = Runtime::new().unwrap();
        b.iter(|| rt.block_on(channel_threads(10000)));
    }
}
