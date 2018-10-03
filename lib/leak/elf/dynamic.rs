use error::*;
use leak::elf::{Class, FieldData};
use leak::Leaker;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DynamicOffsets {
    d_tag: FieldData,
    d_val: FieldData
}

impl DynamicOffsets {
    pub fn d_tag(&self) -> &FieldData { &self.d_tag }
    pub fn d_val(&self) -> &FieldData { &self.d_val }
}

const DYNAMIC_32_OFFSETS: &DynamicOffsets = &DynamicOffsets {
    d_tag: FieldData { offset: 0, length: 4 },
    d_val: FieldData { offset: 4, length: 4 }
};

const DYNAMIC_64_OFFSETS: &DynamicOffsets = &DynamicOffsets {
    d_tag: FieldData { offset: 0, length: 8 },
    d_val: FieldData { offset: 8, length: 8 }
};


pub struct Dynamic<L: Leaker> {
    leaker: Rc<L>,
    base_address: u64,
    class: Class,
    d_tag: RefCell<Option<u64>>,
    d_val: RefCell<Option<u64>>
}

impl<L: Leaker> Dynamic<L> {
    pub fn new(address: u64, class: Class, leaker: Rc<L>) -> Dynamic<L> {
        Dynamic {
            leaker: leaker,
            base_address: address,
            class: class,
            d_tag: RefCell::new(None),
            d_val: RefCell::new(None)
        }
    }

    pub fn leaker(&self) -> &impl Leaker { self.leaker.as_ref() }
    pub fn class(&self) -> &Class { &self.class }
    pub fn base_address(&self) -> u64 { self.base_address }

    pub fn dynamic_offsets(&self) -> &'static DynamicOffsets {
        match self.class() {
            Class::Class32 => DYNAMIC_32_OFFSETS,
            Class::Class64 => DYNAMIC_64_OFFSETS
        }
    }

    pub fn d_tag(&self) -> Result<u64> {
        if self.d_tag.borrow().is_none() {
            self.d_tag
                .replace(
                    Some(self.dynamic_offsets()
                        .d_tag()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.d_tag.borrow().unwrap())
    }

    pub fn d_val(&self) -> Result<u64> {
        if self.d_val.borrow().is_none() {
            self.d_val
                .replace(
                    Some(self.dynamic_offsets()
                        .d_val()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.d_val.borrow().unwrap())
    }
}