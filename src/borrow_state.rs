use std::cell::Cell;
use std::fmt::{Display, Formatter};


#[repr(transparent)]
pub struct BorrowState {
    tag: Cell<isize>
}

impl BorrowState {
    #[inline(always)]
    pub fn unused() -> Self {
        BorrowState{tag: Cell::new(0)}
    }

    #[inline(always)]
    pub fn is_writing(&self) -> bool {
        self.tag.get() < 0
    }
    #[inline(always)]
    pub fn is_reading(&self) -> bool {
       self.tag.get() > 0
    }

    #[inline(always)]
    pub fn is_unused(&self) -> bool {
        self.tag.get() == 0
    }
    #[inline(always)]
    pub fn can_read(&self) -> bool {
        self.tag.get() >= 0
    }

    #[inline(always)]
    pub fn inc_reading(&self) {
        self.tag.set(self.tag.get()+1);
    }

    #[inline(always)]
    pub fn dec_reading(&self) {
        self.tag.set(self.tag.get()-1);
    }


    #[inline(always)]
    pub fn inc_writing(&self) {
        self.tag.set(self.tag.get()-1);
    }

    #[inline(always)]
    pub fn dec_writing(&self) {
        self.tag.set(self.tag.get()+1);
    }
}

impl Display for BorrowState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.tag.get() {
            0 => write!(f,"unused"),
            v if v<0 => write!(f,"borrowed as mut"),
            v => write!(f,"borrowed ({} times)",v)
        }

    }
}