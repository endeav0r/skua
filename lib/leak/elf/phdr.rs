use error::*;
use leak::elf::{Class, FieldData};
use leak::Leaker;
use std::cell::RefCell;
use std::rc::Rc;


pub struct PhdrOffsets {
    p_type:   FieldData,
    p_offset: FieldData,
    p_vaddr:  FieldData,
    p_paddr:  FieldData,
    p_filesz: FieldData,
    p_memsz:  FieldData,
    p_flags:  FieldData,
    p_align:  FieldData,
}

impl PhdrOffsets {
    pub fn p_type(&self)   -> &FieldData { &self.p_type }
    pub fn p_offset(&self) -> &FieldData { &self.p_offset }
    pub fn p_vaddr(&self)  -> &FieldData { &self.p_vaddr }
    pub fn p_paddr(&self)  -> &FieldData { &self.p_paddr }
    pub fn p_filesz(&self) -> &FieldData { &self.p_filesz }
    pub fn p_memsz(&self)  -> &FieldData { &self.p_memsz }
    pub fn p_flags(&self)  -> &FieldData { &self.p_flags }
    pub fn p_align(&self)  -> &FieldData { &self.p_align }
}



const PHDR_32_OFFSETS: &PhdrOffsets = &PhdrOffsets {
    p_type:   FieldData { offset: 0, length: 4},
    p_offset: FieldData { offset: 4, length: 4},
    p_vaddr:  FieldData { offset: 8, length: 4},
    p_paddr:  FieldData { offset: 12, length: 4},
    p_filesz: FieldData { offset: 16, length: 4},
    p_memsz:  FieldData { offset: 20, length: 4},
    p_flags:  FieldData { offset: 24, length: 4},
    p_align:  FieldData { offset: 28, length: 4}
};

const PHDR_64_OFFSETS: &PhdrOffsets = &PhdrOffsets {
    p_type:   FieldData { offset: 0, length: 4},
    p_flags:  FieldData { offset: 4, length: 4},
    p_offset: FieldData { offset: 8, length: 8},
    p_vaddr:  FieldData { offset: 16, length: 8},
    p_paddr:  FieldData { offset: 24, length: 8},
    p_filesz: FieldData { offset: 32, length: 8},
    p_memsz:  FieldData { offset: 40, length: 8},
    p_align:  FieldData { offset: 48, length: 8}
};


pub struct Phdr<L: Leaker> {
    leaker: Rc<L>,
    base_address: u64,
    class: Class,
    p_type: RefCell<Option<u64>>,
    p_offset: RefCell<Option<u64>>,
    p_vaddr: RefCell<Option<u64>>,
    p_paddr: RefCell<Option<u64>>,
    p_filesz: RefCell<Option<u64>>,
    p_memsz: RefCell<Option<u64>>,
    p_flags: RefCell<Option<u64>>,
    p_align: RefCell<Option<u64>>,
}


impl<L: Leaker> Phdr<L> {
    pub fn new(address: u64, class: Class, leaker: Rc<L>) -> Phdr<L> {
        Phdr {
            leaker: leaker,
            base_address: address,
            class:     class,
            p_type:     RefCell::new(None),
            p_offset:   RefCell::new(None),
            p_vaddr:    RefCell::new(None),
            p_paddr:    RefCell::new(None),
            p_filesz:   RefCell::new(None),
            p_memsz:    RefCell::new(None),
            p_flags:    RefCell::new(None),
            p_align: RefCell::new(None),
        }
    }

    pub fn leaker(&self) -> &impl Leaker { self.leaker.as_ref() }
    pub fn class(&self) -> &Class { &self.class }
    pub fn base_address(&self) -> u64 { self.base_address }

    pub fn phdr_offsets(&self) -> &'static PhdrOffsets {
        match self.class() {
            Class::Class32 => PHDR_32_OFFSETS,
            Class::Class64 => PHDR_64_OFFSETS
        }
    }

    pub fn p_type(&self) -> Result<u64> {
        if self.p_type.borrow().is_none() {
            self.p_type
                .replace(
                    Some(self.phdr_offsets()
                        .p_type()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.p_type.borrow().unwrap())
    }

    pub fn p_offset(&self) -> Result<u64> {
        if self.p_offset.borrow().is_none() {
            self.p_offset
                .replace(
                    Some(self.phdr_offsets()
                        .p_offset()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.p_offset.borrow().unwrap())
    }

    pub fn p_vaddr(&self) -> Result<u64> {
        if self.p_vaddr.borrow().is_none() {
            self.p_vaddr
                .replace(
                    Some(self.phdr_offsets()
                        .p_vaddr()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.p_vaddr.borrow().unwrap())
    }

    pub fn p_paddr(&self) -> Result<u64> {
        if self.p_paddr.borrow().is_none() {
            self.p_paddr
                .replace(
                    Some(self.phdr_offsets()
                        .p_paddr()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.p_paddr.borrow().unwrap())
    }

    pub fn p_filesz(&self) -> Result<u64> {
        if self.p_filesz.borrow().is_none() {
            self.p_filesz
                .replace(
                    Some(self.phdr_offsets()
                        .p_filesz()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.p_filesz.borrow().unwrap())
    }

    pub fn p_memsz(&self) -> Result<u64> {
        if self.p_memsz.borrow().is_none() {
            self.p_memsz
                .replace(
                    Some(self.phdr_offsets()
                        .p_memsz()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.p_memsz.borrow().unwrap())
    }

    pub fn p_flags(&self) -> Result<u64> {
        if self.p_flags.borrow().is_none() {
            self.p_flags
                .replace(
                    Some(self.phdr_offsets()
                        .p_flags()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.p_flags.borrow().unwrap())
    }

    pub fn p_align(&self) -> Result<u64> {
        if self.p_align.borrow().is_none() {
            self.p_align
                .replace(
                    Some(self.phdr_offsets()
                        .p_align()
                        .leak(self.base_address(), self.leaker())?));
        }
        Ok(self.p_align.borrow().unwrap())
    }
}