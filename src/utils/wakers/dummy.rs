use core::task::{RawWaker, RawWakerVTable, Waker};

pub(crate) fn dummy_waker() -> Waker {
    fn new_raw_waker() -> RawWaker {
        unsafe fn no_op(_data: *const ()) {}
        unsafe fn clone(_data: *const ()) -> RawWaker {
            new_raw_waker()
        }
        RawWaker::new(
            &0usize as *const usize as *const (),
            &RawWakerVTable::new(clone, no_op, no_op, no_op),
        )
    }
    unsafe { Waker::from_raw(new_raw_waker()) }
}
