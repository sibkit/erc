#![feature(dispatch_from_dyn)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(allocator_api)]
extern crate core;



mod erc;
mod erc_box;
mod ewc;
mod ref_holders;
mod borrow_state;
mod borrow_error;



#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::{Rc, Weak};
    use crate::borrow_error::BorrowError;
    use crate::erc::Erc;
    use super::*;

    struct Zero;

    #[test]
    fn addr_test() {
        let a = ();
        println!("addr of a: {:?}", &a as *const ());

        let b = Zero {};
        println!("addr of b: {:?}", &b as *const Zero);

        let c = Box::new(b);
        println!("addr of c: {:?}", c.deref() as *const Zero);

        let d = Box::new(());
        println!("addr of d: {:?}", d.deref() as *const ());
        println!("addr of null: {:?}", std::ptr::null_mut::<String>());

        let w = Weak::<String>::new();
        println!("addr of empty weak: {:?}", w.as_ptr());
    }

    #[test]
    fn new_cyclic_test() {
        use std::rc::{Rc, Weak};
        struct Gadget {
            name: String,
            me2: Weak<Gadget>,
        }

        impl Gadget {
            /// Construct a reference counted Gadget.
            fn new(name: String) -> Rc<Self> {
                // `me` is a `Weak<Gadget>` pointing at the new allocation of the
                // `Rc` we're constructing.
                Rc::new_cyclic(|me| {
                    // Create the actual struct here.
                    Gadget { name, me2: me.clone() }
                })
            }


            /// Return a reference counted pointer to Self.
            fn me(&self) -> Rc<Self> {
                self.me2.upgrade().unwrap()
            }
        }

        let g = Gadget::new("Ola".to_string());
    }

    #[test]
    fn erc_borrow_test() {
        let val:Erc<String> = Erc::new(String::from("Epilaboda harbana violcated"));

        let ref_holder = val.try_borrow().unwrap();
        debug_assert!(val.try_borrow_mut().is_err(),"try_borrow_mut invalid");
        println!("{}", ref_holder.deref());
        println!("state: {}", val.get_state());
        println!("{}", ref_holder.deref());
        println!("state: {}", val.get_state());
    }

    #[test]
    fn erc_dyn_test(){
        let mut b_vec: Vec<Box<dyn Any>> = vec!();
        b_vec.push(Box::new(String::from("Astra")));

        let mut rc_vec: Vec<Rc<dyn Any>> = vec!();
        rc_vec.push(Rc::new(String::from("Astra")));

        let rc = Rc::new(String::from("Astra")) as Rc<dyn Any>;
        let erc = Erc::new(String::from("Astra")) as Erc<dyn Any>;

       // let mut vec: Vec<Erc<dyn Any>> = vec!();
       // vec.push(Erc::new(String::from("Astra")));
       //vec.push(Erc::new(56u32));
        //vec.push(Erc::new(Zero{}));

    }
}
