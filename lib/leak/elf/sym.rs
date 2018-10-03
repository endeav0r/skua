use error::*;
use leak::elf::{Class, FieldData};
use leak::Leaker;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SymOffsets {
    st_name:  FieldData,
    st_value: FieldData,
    st_size:  FieldData,
    st_info:  FieldData,
    st_other: FieldData,
    st_shndx: FieldData
}

impl SymOffsets {
    pub fn st_name(&self) -> &FieldData { &self.st_name }
    pub fn st_value(&self) -> &FieldData { &self.st_value }
    pub fn st_size(&self) -> &FieldData { &self.st_size }
    pub fn st_info(&self) -> &FieldData { &self.st_info }
    pub fn st_other(&self) -> &FieldData { &self.st_other }
    pub fn st_shndx(&self) -> &FieldData { &self.st_shndx }
}

const SYM_32_OFFSETS: &SymOffsets = &SymOffsets {
    st_name: FieldData  { offset: 0, length: 4},
    st_value: FieldData { offset: 4, length: 4},
    st_size: FieldData  { offset: 8, length: 4},
    st_info: FieldData  { offset: 12, length: 1},
    st_other: FieldData { offset: 13, length: 1},
    st_shndx: FieldData { offset: 14, length: 2},
};

const SYM_64_OFFSETS: &SymOffsets = &SymOffsets {
    st_name: FieldData  { offset: 0, length: 4},
    st_info: FieldData  { offset: 4, length: 1},
    st_other: FieldData { offset: 5, length: 1},
    st_shndx: FieldData { offset: 6, length: 2},
    st_value: FieldData { offset: 8, length: 8},
    st_size: FieldData  { offset: 16, length: 8},
};


pub struct Sym<L: Leaker> {
    leaker: Rc<L>,
    base_address: u64,
    class: Class,
    name: RefCell<Option<String>>,
    st_name:  RefCell<Option<u64>>,
    st_value: RefCell<Option<u64>>,
    st_size:  RefCell<Option<u64>>,
    st_info:  RefCell<Option<u64>>,
    st_other: RefCell<Option<u64>>,
    st_shndx: RefCell<Option<u64>>,
}


impl<L: Leaker> Sym<L> {
    pub fn new(address: u64, class: Class, leaker: Rc<L>) -> Sym<L> {
        Sym {
            leaker: leaker,
            base_address: address,
            class: class,
            name: RefCell::new(None),
            st_name:  RefCell::new(None),
            st_value: RefCell::new(None),
            st_size:  RefCell::new(None),
            st_info:  RefCell::new(None),
            st_other: RefCell::new(None),
            st_shndx: RefCell::new(None)
        }
    }

    pub fn leaker(&self) -> &impl Leaker { self.leaker.as_ref() }
    pub fn base_address(&self) -> u64 { self.base_address }
    pub fn class(&self) -> &Class { &self.class }



    pub fn sym_offsets(&self) -> &'static SymOffsets {
        match self.class() {
            Class::Class32 => SYM_32_OFFSETS,
            Class::Class64 => SYM_64_OFFSETS
        }
    }

    pub fn name(&self, strtab: u64) -> Result<String> {
        if self.name.borrow().is_none() {
            let address = strtab + self.st_name()?;
            self.name.replace(Some(self.leaker().leak_string(address)?));
        }
        Ok(self.name.borrow().clone().unwrap())
    }

    pub fn st_name(&self) -> Result<u64> {
        if self.st_name.borrow().is_none() {
            self.st_name
                .replace(
                    Some(self.sym_offsets()
                        .st_name()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.st_name.borrow().unwrap())
    }

    pub fn st_value(&self) -> Result<u64> {
        if self.st_value.borrow().is_none() {
            self.st_value
                .replace(
                    Some(self.sym_offsets()
                        .st_value()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.st_value.borrow().unwrap())
    }

    pub fn st_size(&self) -> Result<u64> {
        if self.st_size.borrow().is_none() {
            self.st_size
                .replace(Some(self.sym_offsets()
                    .st_size()
                    .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.st_size.borrow().unwrap())
    }

    pub fn st_info(&mut self) -> Result<u64> {
        if self.st_info.borrow().is_none() {
            self.st_info
                .replace(
                    Some(self.sym_offsets()
                        .st_info()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.st_info.borrow().unwrap())
    }

    pub fn st_other(&mut self) -> Result<u64> {
        if self.st_other.borrow().is_none() {
            self.st_other
                .replace(
                    Some(self.sym_offsets()
                        .st_other()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.st_other.borrow().unwrap())
    }

    pub fn st_shndx(&mut self) -> Result<u64> {
        if self.st_shndx.borrow().is_none() {
            self.st_shndx
                .replace(
                    Some(self.sym_offsets()
                        .st_shndx()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.st_shndx.borrow().unwrap())
    }
}