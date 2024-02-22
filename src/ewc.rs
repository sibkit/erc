use std::ptr;
use std::rc::{Rc, Weak};
use crate::erc::Erc;
use crate::erc_box::ErcBox;

pub struct Ewc<T:?Sized> {
    ptr: *mut ErcBox<T>
}

impl <T> Ewc<T>{
    #[inline]
    pub const fn new() -> Ewc<T> {
        Ewc { ptr: usize::MAX as *mut ErcBox<T> }
    }
}

impl <T:?Sized> Ewc<T> {
    pub fn upgrade(&self) -> Option<Erc<T>>
    {
        let strong = (unsafe { &*self.ptr }).strong.get();
        if strong == 0 {
            None
        } else {
            unsafe {
                let strong = strong.wrapping_add(1);
                (&*self.ptr).strong.set(strong);
                Some(Erc::from_erc_box_ptr(self.ptr))
            }
        }
    }
}

impl<T: ?Sized> Clone for Ewc<T> {
    /// Makes a clone of the `Weak` pointer that points to the same allocation.
    ///
    /// # Examples
    ///
    /// ```
    /// use erc::{Erc, Ewc};
    ///
    /// let weak_five = Erc::downgrade(&Erc::new(5));
    ///
    /// let _ = Ewc::clone(&weak_five);
    /// ```
    #[inline]
    fn clone(&self) -> Ewc<T> {
        if !self.ptr.is_null() {
            let weak_cell = unsafe { &(*self.ptr).weak };
            weak_cell.set(weak_cell.get()+1);
            Ewc { ptr: self.ptr }
        }
        else { Ewc { ptr: self.ptr } }
    }
}