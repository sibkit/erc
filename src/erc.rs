use std::alloc::Allocator;
use std::cell::{Cell, UnsafeCell};
use std::fmt::{Debug, Formatter};
use std::marker::{PhantomData, Unsize};
use std::ops::{CoerceUnsized, DispatchFromDyn};
use std::ptr::NonNull;
use std::rc::Rc;
use crate::borrow_error::BorrowError;
use crate::borrow_state::BorrowState;

use crate::erc_box::{ErcBox};
use crate::ref_holders::{RefHolder, RefMutHolder};


pub struct Erc<T:?Sized> {
    pub(crate) ptr: *mut ErcBox<T>,
    phantom: PhantomData<T>
}

impl <T> Erc<T> {
    #[inline(always)]
    pub fn new(x: T) -> Self {
        Erc {
            ptr: Box::leak(Box::new(
                ErcBox::new(x))),
            phantom: Default::default()
        }
    }
}

impl <T: ?Sized> Erc<T> {



    pub fn try_borrow(&self) -> Result<RefHolder<'_, T>, BorrowError> {
        let erc_box = unsafe { &*self.ptr };
        let mut state = &erc_box.borrow_state();
        if state.can_read() {
            let value = RefHolder::from_erc_box(erc_box);
            Ok(value)
        } else {
            Err(BorrowError {})
        }
    }

    pub fn try_borrow_mut(&self) -> Result<RefMutHolder<'_,T>, BorrowError>{
        let erc_box = unsafe { &*self.ptr };
        let state = &erc_box.borrow_state();
        if state.is_unused() {
            let value = RefMutHolder::from_erc_box_ptr(self.ptr);
            Ok(value)
        } else {
            Err(BorrowError {})
        }
    }

    pub fn get_state(&self) -> &BorrowState{
        unsafe { (&*self.ptr).borrow_state() }
    }

    pub(crate) fn from_erc_box_ptr(ptr:  *mut ErcBox<T>) -> Self {
        Erc {
            ptr,
            phantom: Default::default()
        }
    }
}

impl<T: Debug> Debug for Erc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Erc");
        match self.try_borrow() {
            Ok(borrow) => d.field("ptr", &borrow),
            Err(_) => d.field("ptr", &format_args!("<borrowed>")),
        };
        d.finish()
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Erc<U>> for Erc<T> {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Erc<U>> for Erc<T> {}