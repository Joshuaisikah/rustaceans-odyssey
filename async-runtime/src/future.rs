use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ─── Ready ────────────────────────────────────────────────────────────────────
//
// A future that resolves on the very first poll, returning the stored value.
// Hint: store the value in an Option<T> and take() it inside poll().

pub struct Ready<T>(/* TODO: store the value */ std::marker::PhantomData<T>);

pub fn ready<T>(val: T) -> Ready<T> {
    let _ = val;
    todo!()
}

impl<T: Unpin> Future for Ready<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        todo!()
    }
}

// ─── YieldNow ────────────────────────────────────────────────────────────────
//
// A future that returns Poll::Pending exactly once, then Poll::Ready(()).
// Hint: use a `bool` field to track whether it has already yielded.

pub struct YieldNow(/* TODO: bool field */);

pub fn yield_now() -> YieldNow {
    todo!()
}

impl Future for YieldNow {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_noop_cx() -> (std::task::Waker, std::task::Context<'static>) {
        use std::task::{RawWaker, RawWakerVTable, Waker};
        fn clone(_: *const ()) -> RawWaker { raw() }
        fn noop(_: *const ()) {}
        fn raw() -> RawWaker {
            static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
            RawWaker::new(std::ptr::null(), &VT)
        }
        let waker = unsafe { Waker::from_raw(raw()) };
        // SAFETY: waker outlives cx in this test scope
        let cx = unsafe {
            std::mem::transmute::<Context<'_>, Context<'static>>(Context::from_waker(&waker))
        };
        (waker, cx)
    }

    // Ready resolves on the first poll.
    #[test]
    fn test_ready_resolves_immediately() {
        let (_, mut cx) = make_noop_cx();
        let mut f = Box::pin(ready(42i32));
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(v) => assert_eq!(v, 42),
            Poll::Pending  => panic!("Ready should never return Pending"),
        }
    }

    // Ready works with any T, including String.
    #[test]
    fn test_ready_with_string() {
        let (_, mut cx) = make_noop_cx();
        let mut f = Box::pin(ready("hello".to_string()));
        assert!(matches!(f.as_mut().poll(&mut cx), Poll::Ready(s) if s == "hello"));
    }

    // YieldNow returns Pending on the first poll.
    #[test]
    fn test_yield_now_first_poll_is_pending() {
        let (_, mut cx) = make_noop_cx();
        let mut f = Box::pin(yield_now());
        assert_eq!(f.as_mut().poll(&mut cx), Poll::Pending, "first poll must be Pending");
    }

    // YieldNow returns Ready on the second poll.
    #[test]
    fn test_yield_now_second_poll_is_ready() {
        let (_, mut cx) = make_noop_cx();
        let mut f = Box::pin(yield_now());
        let _ = f.as_mut().poll(&mut cx); // first: Pending
        assert_eq!(
            f.as_mut().poll(&mut cx),
            Poll::Ready(()),
            "second poll must be Ready"
        );
    }

    // YieldNow stays Ready after the second poll (not cycling back to Pending).
    #[test]
    fn test_yield_now_stays_ready_after_second_poll() {
        let (_, mut cx) = make_noop_cx();
        let mut f = Box::pin(yield_now());
        let _ = f.as_mut().poll(&mut cx);
        let _ = f.as_mut().poll(&mut cx);
        // polling a completed future is valid — implementation may panic or return Ready
        // but must not return Pending again
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            f.as_mut().poll(&mut cx)
        }));
        if let Ok(p) = result {
            assert_ne!(p, Poll::Pending, "must not cycle back to Pending");
        }
    }
}
