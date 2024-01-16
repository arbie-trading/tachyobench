macro_rules! bench {
    ($channel_name:ident) => {
        pub mod $channel_name {
            use std::num::NonZeroU32;
            use std::time::Instant;

            use crate::channel_shims::$channel_name::channel;
            use crate::executor_shims::Executor;
            use crate::{BenchIterator, BenchResult};

            pub fn bench<E: Executor>(samples: NonZeroU32) -> BenchIterator {
                const MESSAGES_PER_CHANNEL: usize = 1_000_000;
                const CHANNELS: usize = 61;
                let total_messages =
                    MESSAGES_PER_CHANNEL * CHANNELS;

                let results = [10_000, 100_000]
                    .into_iter()
                    .map(move |capacity: usize| {
                        let throughput: Vec<_> = (0..samples.get())
                            .map(|_| {
                                let mut executor = E::default();

                                for _ in 0..CHANNELS {
                                    let (mut s, mut r) = channel(capacity);

                                    let _ = executor.spawn(async move {
                                        for i in 0..MESSAGES_PER_CHANNEL {
                                            s.send(i).await;
                                        }
                                    });

                                    executor.spawn(async move {
                                        for _ in 0..MESSAGES_PER_CHANNEL
                                        {
                                            _ = r.recv().await;
                                        }
                                    })
                                }

                                let start_time = Instant::now();
                                executor.join_all();
                                let duration = Instant::now() - start_time;

                                total_messages as f64 / duration.as_secs_f64()
                            })
                            .collect();

                        BenchResult::new(String::from("capacity"), capacity.to_string(), throughput)
                    });

                Box::new(results)
            }
        }
    };
}

crate::macros::add_spsc_bench!();
