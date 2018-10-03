use error::*;
use leak::elf::{Class, FieldData};
use leak::Leaker;
use std::cell::RefCell;
use std::rc::Rc;


pub struct ShdrOffsets {
    sh_name:      FieldData,
    sh_type:      FieldData,
    sh_flags:     FieldData,
    sh_addr:      FieldData,
    sh_offset:    FieldData,
    sh_size:      FieldData,
    sh_link:      FieldData,
    sh_info:      FieldData,
    sh_addralign: FieldData,
    sh_entsize:   FieldData
}

impl ShdrOffsets {
    pub fn sh_name(&self)      -> &FieldData { &self.sh_name }
    pub fn sh_type(&self)      -> &FieldData { &self.sh_type }
    pub fn sh_flags(&self)     -> &FieldData { &self.sh_flags }
    pub fn sh_addr(&self)      -> &FieldData { &self.sh_addr }
    pub fn sh_offset(&self)    -> &FieldData { &self.sh_offset }
    pub fn sh_size(&self)      -> &FieldData { &self.sh_size }
    pub fn sh_link(&self)      -> &FieldData { &self.sh_link }
    pub fn sh_info(&self)      -> &FieldData { &self.sh_info }
    pub fn sh_addralign(&self) -> &FieldData { &self.sh_addralign }
    pub fn sh_entsize(&self)   -> &FieldData { &self.sh_entsize }
}



const SHDR_32_OFFSETS: &ShdrOffsets = &ShdrOffsets {
    sh_name: FieldData      { offset: 0, length: 4},
    sh_type: FieldData      { offset: 4, length: 4},
    sh_flags: FieldData     { offset: 8, length: 4},
    sh_addr: FieldData      { offset: 12, length: 4},
    sh_offset: FieldData    { offset: 16, length: 4},
    sh_size: FieldData      { offset: 20, length: 4},
    sh_link: FieldData      { offset: 24, length: 4},
    sh_info: FieldData      { offset: 28, length: 4},
    sh_addralign: FieldData { offset: 32, length: 4},
    sh_entsize: FieldData   { offset: 36, length: 4}
};

const SHDR_64_OFFSETS: &ShdrOffsets = &ShdrOffsets {
    sh_name: FieldData      { offset: 0, length: 4},
    sh_type: FieldData      { offset: 4, length: 4},
    sh_flags: FieldData     { offset: 8, length: 8},
    sh_addr: FieldData      { offset: 16, length: 8},
    sh_offset: FieldData    { offset: 24, length: 8},
    sh_size: FieldData      { offset: 32, length: 8},
    sh_link: FieldData      { offset: 40, length: 4},
    sh_info: FieldData      { offset: 44, length: 4},
    sh_addralign: FieldData { offset: 48, length: 8},
    sh_entsize: FieldData   { offset: 56, length: 8}
};


pub struct Shdr<L: Leaker> {
    leaker: Rc<L>,
    base_address: u64,
    class: Class,
    sh_name: RefCell<Option<u64>>,
    sh_type: RefCell<Option<u64>>,
    sh_flags: RefCell<Option<u64>>,
    sh_addr: RefCell<Option<u64>>,
    sh_offset: RefCell<Option<u64>>,
    sh_size: RefCell<Option<u64>>,
    sh_link: RefCell<Option<u64>>,
    sh_info: RefCell<Option<u64>>,
    sh_addralign: RefCell<Option<u64>>,
    sh_entsize: RefCell<Option<u64>>
}


impl<L: Leaker> Shdr<L> {
    pub fn new(address: u64, class: Class, leaker: Rc<L>) -> Shdr<L> {
        Shdr {
            leaker: leaker,
            base_address: address,
            class:     class,
            sh_name:   RefCell::new(None),
            sh_type:   RefCell::new(None),
            sh_flags:  RefCell::new(None),
            sh_addr:   RefCell::new(None),
            sh_offset: RefCell::new(None),
            sh_size:   RefCell::new(None),
            sh_link:   RefCell::new(None),
            sh_info:   RefCell::new(None),
            sh_addralign: RefCell::new(None),
            sh_entsize: RefCell::new(None)
        }
    }

    pub fn leaker(&self) -> &impl Leaker { self.leaker.as_ref() }
    pub fn class(&self) -> &Class { &self.class }
    pub fn base_address(&self) -> u64 { self.base_address }

    pub fn shdr_offsets(&self) -> &'static ShdrOffsets {
        match self.class() {
            Class::Class32 => SHDR_32_OFFSETS,
            Class::Class64 => SHDR_64_OFFSETS
        }
    }

    pub fn sh_name(&self) -> Result<u64> {
        if self.sh_name.borrow().is_none() {
            self.sh_name
                .replace(
                    Some(self.shdr_offsets()
                        .sh_name()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_name.borrow().unwrap())
    }

    pub fn sh_type(&self) -> Result<u64> {
        if self.sh_type.borrow().is_none() {
            self.sh_type
                .replace(
                    Some(self.shdr_offsets()
                        .sh_type()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_type.borrow().unwrap())
    }

    pub fn sh_flags(&self) -> Result<u64> {
        if self.sh_flags.borrow().is_none() {
            self.sh_flags
                .replace(
                    Some(self.shdr_offsets()
                        .sh_flags()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_flags.borrow().unwrap())
    }

    pub fn sh_addr(&self) -> Result<u64> {
        if self.sh_addr.borrow().is_none() {
            self.sh_addr
                .replace(
                    Some(self.shdr_offsets()
                        .sh_addr()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_addr.borrow().unwrap())
    }

    pub fn sh_offset(&self) -> Result<u64> {
        if self.sh_offset.borrow().is_none() {
            self.sh_offset
                .replace(
                    Some(self.shdr_offsets()
                        .sh_offset()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_offset.borrow().unwrap())
    }

    pub fn sh_size(&self) -> Result<u64> {
        if self.sh_size.borrow().is_none() {
            self.sh_size
                .replace(
                    Some(self.shdr_offsets()
                        .sh_size()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_size.borrow().unwrap())
    }

    pub fn sh_link(&self) -> Result<u64> {
        if self.sh_link.borrow().is_none() {
            self.sh_link
                .replace(
                    Some(self.shdr_offsets()
                        .sh_link()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_link.borrow().unwrap())
    }

    pub fn sh_info(&self) -> Result<u64> {
        if self.sh_info.borrow().is_none() {
            self.sh_info
                .replace(
                    Some(self.shdr_offsets()
                        .sh_info()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_info.borrow().unwrap())
    }

    pub fn sh_addralign(&self) -> Result<u64> {
        if self.sh_addralign.borrow().is_none() {
            self.sh_addralign
                .replace(
                    Some(self.shdr_offsets()
                        .sh_addralign()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_addralign.borrow().unwrap())
    }

    pub fn sh_entsize(&self) -> Result<u64> {
        if self.sh_entsize.borrow().is_none() {
            self.sh_entsize
                .replace(
                    Some(self.shdr_offsets()
                        .sh_entsize()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.sh_entsize.borrow().unwrap())
    }
}