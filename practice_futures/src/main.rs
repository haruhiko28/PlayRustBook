use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

enum State {
    Ready,
    Busy,
    Completed,
}

struct TimeFuture {
    duration: u64,
    state: Arc<Mutex<State>>,
}

impl TimeFuture {
    fn new(duration: u64) -> Self {
        Self {
            duration,
            state: Arc::new(Mutex::new(State::Ready)),
        }
    }
}

impl Future for TimeFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Ready => {
                *state = State::Busy;
                let cloned_state = self.state.clone();
                let duration = self.duration;
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(duration));
                    println!("finished. {}", duration);
                    let mut s = cloned_state.lock().unwrap();
                    *s = State::Completed;
                });
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            State::Busy => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            State::Completed => Poll::Ready(()),
        }
    }
}

fn main() {
    let future1 = TimeFuture::new(1000);
    let future2 = TimeFuture::new(500);
    let future3 = TimeFuture::new(100);
    let futures = join_all(vec![future1,future2,future3]);
    executor::block_on(futures);
}
