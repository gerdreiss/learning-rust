use futures::future::Future;
use futures::task::{Context, Poll};

use std::pin::Pin;

pub struct SimpleFuture {
    n: i32,
}

impl Future for SimpleFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    #[test]
    fn it_works() {
        let f = SimpleFuture { n: 10 };
        let v = block_on(f);
        assert_eq!(v, 10);
    }
}
