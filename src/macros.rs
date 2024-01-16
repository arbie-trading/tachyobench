macro_rules! add_bench {
    () => {
        bench!(async_channel);
        bench!(flume);
        bench!(futures_mpsc);
        bench!(tachyonix);
        // bench!(thingbuf);
        // bench!(postage_mpsc);
        bench!(tokio_mpsc);
        bench!(tokio_broadcast);
        bench!(kanal);
    };
}

macro_rules! add_spsc_bench {
    () => {
        bench!(async_channel);
        bench!(flume);
        bench!(futures_mpsc);
        bench!(tachyonix);
        // bench!(thingbuf);
        // bench!(postage_mpsc);
        bench!(tokio_mpsc);
        bench!(tokio_broadcast);
        bench!(kanal);
        bench!(async_ringbuf);
    };
}


pub(crate) use add_bench;
pub(crate) use add_spsc_bench;
