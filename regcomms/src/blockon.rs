use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(core::ptr::null(), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

/// Minimal `block_on` for no_std
pub fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);
    let mut pinned = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Ready(val) => return val,
            Poll::Pending => (),
        }
    }
}
