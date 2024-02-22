use std::cell::{Cell, UnsafeCell};
use crate::borrow_state::BorrowState;


pub(crate) struct ErcBox<T:?Sized> {
    pub(crate) strong: Cell<usize>,
    pub(crate) weak: Cell<usize>,
    borrow_state: BorrowState,
    pub(crate) value: T
}

impl <T> ErcBox<T> {
    pub fn new(value: T) -> Self{
        ErcBox {
            strong: Cell::new(1),
            weak: Cell::new(1),
            borrow_state: BorrowState::unused(),
            value,
        }
    }
}

impl <T:?Sized> ErcBox<T> {
    pub fn get_raw_ptr(&self) -> *mut T {
        &self.value as *const T as *mut T
    }

    pub fn borrow_state(&self) -> &BorrowState {
        &self.borrow_state
    }
}

