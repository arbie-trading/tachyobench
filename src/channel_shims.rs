pub mod async_channel {
    use ::async_channel as channel;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await.ok()
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::bounded(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod flume {
    use ::flume as channel;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send_async(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv_async().await.ok()
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::bounded(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod futures_mpsc {
    use ::futures_channel::mpsc as channel;
    use ::futures_util::sink::SinkExt;
    use ::futures_util::stream::StreamExt;

    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.next().await
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod postage_mpsc {
    use ::postage::mpsc as channel;
    use ::postage::sink::Sink;
    use ::postage::stream::Stream;

    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod tachyonix {
    use ::tachyonix as channel;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: std::fmt::Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await.ok()
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod thingbuf {
    use ::thingbuf::mpsc as channel;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: std::fmt::Debug + Default + Clone> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T: Default + Clone> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await
        }
    }

    pub fn channel<T: Default + Clone>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod tokio_mpsc {
    use ::tokio::sync::mpsc as channel;

    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}


pub mod tokio_broadcast {
    use ::tokio::sync::broadcast as channel;

    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).unwrap();
        }
    }

    pub struct Receiver<T: Clone> {
        inner: channel::Receiver<T>,
    }
    impl<T: Clone> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await.ok()
        }
    }

    pub fn channel<T: Clone>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod async_ringbuf {
    use std::{mem::MaybeUninit, sync::Arc};

    use async_ringbuf::AsyncRb;
    use async_ringbuf::traits::AsyncProducer;
    use async_ringbuf::traits::AsyncConsumer;
    use async_ringbuf::traits::Split;

    pub struct Sender<T> {
        inner: async_ringbuf::wrap::AsyncWrap<
            Arc<AsyncRb<Vec<MaybeUninit<T>>>>,
            true,
            false,
        >
    }
    impl<T: std::fmt::Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.push(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: async_ringbuf::wrap::AsyncWrap<
            Arc<AsyncRb<Vec<MaybeUninit<T>>>>,
            false,
            true,
        >,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.pop().await
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let rb = async_ringbuf::AsyncHeapRb::<T>::new(capacity);
        let (prod, cons) = rb.split();
        (Sender { inner: prod }, Receiver { inner: cons })
    }
}

pub mod kanal {

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: kanal::AsyncSender<T>,
    }
    impl<T> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: kanal::AsyncReceiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await.ok()
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = kanal::bounded_async(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}