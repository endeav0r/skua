use error::*;
use leak::elf::{Class, FieldData};
use leak::Leaker;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LinkMapOffsets {
    l_addr: FieldData,
    l_name: FieldData,
    l_ld:   FieldData,
    l_next:  FieldData,
    l_prev:  FieldData
}

impl LinkMapOffsets {
    pub fn l_addr(&self) -> &FieldData { &self.l_addr }
    pub fn l_name(&self) -> &FieldData { &self.l_name }
    pub fn l_ld(&self)   -> &FieldData { &self.l_ld }
    pub fn l_next(&self) -> &FieldData { &self.l_next }
    pub fn l_prev(&self) -> &FieldData { &self.l_prev }
}

const LINK_MAP_32_OFFSETS: &LinkMapOffsets = &LinkMapOffsets {
    l_addr: FieldData { offset: 0, length: 4 },
    l_name: FieldData { offset: 4, length: 4 },
    l_ld:   FieldData { offset: 8, length: 4 },
    l_next: FieldData { offset: 12, length: 4 },
    l_prev: FieldData { offset: 16, length: 4 }
};

const LINK_MAP_64_OFFSETS: &LinkMapOffsets = &LinkMapOffsets {
    l_addr: FieldData { offset: 0, length: 8 },
    l_name: FieldData { offset: 8, length: 8 },
    l_ld:   FieldData { offset: 16, length: 8 },
    l_next: FieldData { offset: 24, length: 8 },
    l_prev: FieldData { offset: 32, length: 8 }
};


pub struct LinkMap<L: Leaker> {
    leaker: Rc<L>,
    base_address: u64,
    class: Class,
    l_addr: RefCell<Option<u64>>,
    l_name: RefCell<Option<u64>>,
    l_ld: RefCell<Option<u64>>,
    l_next: RefCell<Option<u64>>,
    l_prev: RefCell<Option<u64>>
}

impl<L: Leaker> LinkMap<L> {
    pub fn new(address: u64, class: Class, leaker: Rc<L>) -> LinkMap<L> {
        LinkMap {
            leaker: leaker,
            base_address: address,
            class: class,
            l_addr: RefCell::new(None),
            l_name: RefCell::new(None),
            l_ld: RefCell::new(None),
            l_next: RefCell::new(None),
            l_prev: RefCell::new(None),
        }
    }

    pub fn leaker(&self) -> &impl Leaker { self.leaker.as_ref() }
    pub fn class(&self) -> &Class { &self.class }
    pub fn base_address(&self) -> u64 { self.base_address }

    pub fn dynamic_offsets(&self) -> &'static LinkMapOffsets {
        match self.class() {
            Class::Class32 => LINK_MAP_32_OFFSETS,
            Class::Class64 => LINK_MAP_64_OFFSETS
        }
    }

    pub fn l_addr(&self) -> Result<u64> {
        if self.l_addr.borrow().is_none() {
            self.l_addr
                .replace(
                    Some(self.dynamic_offsets()
                        .l_addr()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.l_addr.borrow().unwrap())
    }

    pub fn l_name(&self) -> Result<u64> {
        if self.l_name.borrow().is_none() {
            self.l_name
                .replace(
                    Some(self.dynamic_offsets()
                        .l_name()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.l_name.borrow().unwrap())
    }

    pub fn l_ld(&self) -> Result<u64> {
        if self.l_ld.borrow().is_none() {
            self.l_ld
                .replace(
                    Some(self.dynamic_offsets()
                        .l_ld()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.l_ld.borrow().unwrap())
    }

    pub fn l_next(&self) -> Result<u64> {
        if self.l_next.borrow().is_none() {
            self.l_next
                .replace(
                    Some(self.dynamic_offsets()
                        .l_next()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.l_next.borrow().unwrap())
    }

    pub fn l_prev(&self) -> Result<u64> {
        if self.l_prev.borrow().is_none() {
            self.l_prev
                .replace(
                    Some(self.dynamic_offsets()
                        .l_prev()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.l_prev.borrow().unwrap())
    }
}