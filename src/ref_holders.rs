use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use crate::erc::Erc;
use crate::erc_box::ErcBox;

pub struct RefHolder<'a, T:?Sized> {
    erc_box: &'a ErcBox<T>,
}

impl <'a, T:?Sized> RefHolder<'a,T> {
    pub(crate) fn from_erc_box(erc_box: &'a ErcBox<T>)->Self {
        let bsc = &mut unsafe { &*(erc_box as *const ErcBox<T> as *mut ErcBox<T>) };
        bsc.borrow_state().inc_reading();
        RefHolder {erc_box}
    }
}

impl <'a,T:?Sized> Drop for RefHolder<'a,T> {
    fn drop(&mut self) {
        let bsc = &mut unsafe { &*(self.erc_box as *const ErcBox<T> as *mut ErcBox<T>) };
         bsc.borrow_state().dec_reading();
    }
}

impl <'a,T> Debug for RefHolder<'a,T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl <'a,T> Deref for RefHolder<'a,T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.erc_box.value
    }
}

pub struct RefMutHolder<'a,T:?Sized> {
    erc_box_ptr: *mut ErcBox<T>,
    phantom_data: PhantomData<&'a T>
}

impl <'a, T:?Sized> RefMutHolder<'a,T> {
    pub(crate) fn from_erc_box_ptr(erc_box_ptr: *mut ErcBox<T>)->Self {
        let bsc = &mut unsafe { &*erc_box_ptr };
        bsc.borrow_state().inc_writing();
        RefMutHolder {erc_box_ptr, phantom_data: Default::default() }
    }
}

impl <'a,T:?Sized> Drop for RefMutHolder<'a,T> {
    fn drop(&mut self) {
        let bsc = &mut unsafe { &*self.erc_box_ptr };
        bsc.borrow_state().dec_writing();
    }
}