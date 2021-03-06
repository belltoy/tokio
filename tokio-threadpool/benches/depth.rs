#![feature(test)]

extern crate futures;
extern crate futures_pool;
extern crate futures_cpupool;
extern crate num_cpus;
extern crate test;

const ITER: usize = 20_000;

mod us {
    use futures::future::{self, Executor};
    use futures_pool::*;
    use test;
    use std::sync::mpsc;

    #[bench]
    fn chained_spawn(b: &mut test::Bencher) {
        let (sched_tx, _scheduler) = Pool::new();

        fn spawn(sched_tx: Sender, res_tx: mpsc::Sender<()>, n: usize) {
            if n == 0 {
                res_tx.send(()).unwrap();
            } else {
                let sched_tx2 = sched_tx.clone();
                sched_tx.execute(future::lazy(move || {
                    spawn(sched_tx2, res_tx, n - 1);
                    Ok(())
                })).ok().unwrap();
            }
        }

        b.iter(move || {
            let (res_tx, res_rx) = mpsc::channel();

            spawn(sched_tx.clone(), res_tx, super::ITER);
            res_rx.recv().unwrap();
        });
    }
}

mod cpupool {
    use futures::future::{self, Executor};
    use futures_cpupool::*;
    use num_cpus;
    use test;
    use std::sync::mpsc;

    #[bench]
    fn chained_spawn(b: &mut test::Bencher) {
        let pool = CpuPool::new(num_cpus::get());

        fn spawn(pool: CpuPool, res_tx: mpsc::Sender<()>, n: usize) {
            if n == 0 {
                res_tx.send(()).unwrap();
            } else {
                let pool2 = pool.clone();
                pool.execute(future::lazy(move || {
                    spawn(pool2, res_tx, n - 1);
                    Ok(())
                })).ok().unwrap();
            }
        }

        b.iter(move || {
            let (res_tx, res_rx) = mpsc::channel();

            spawn(pool.clone(), res_tx, super::ITER);
            res_rx.recv().unwrap();
        });
    }
}
