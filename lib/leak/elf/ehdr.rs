use error::*;
use leak::elf::{Class, Endian, FieldData};
use leak::Leaker;
use std::cell::RefCell;
use std::rc::Rc;


pub struct EhdrOffsets {
    e_type:      FieldData,
    e_entry:     FieldData,
    e_phoff:     FieldData,
    e_phentsize: FieldData,
    e_phnum:     FieldData,
    e_shoff:     FieldData,
    e_shentsize: FieldData,
    e_shnum:     FieldData,
    e_shstrndx:  FieldData
}

impl EhdrOffsets {
    pub fn e_type(&self)      -> &FieldData { &self.e_type }
    pub fn e_entry(&self)     -> &FieldData { &self.e_entry }
    pub fn e_phoff(&self)     -> &FieldData { &self.e_phoff }
    pub fn e_shoff(&self)     -> &FieldData { &self.e_shoff }
    pub fn e_phentsize(&self) -> &FieldData { &self.e_phentsize }
    pub fn e_shentsize(&self) -> &FieldData { &self.e_shentsize }
    pub fn e_phnum(&self)     -> &FieldData { &self.e_phnum }
    pub fn e_shnum(&self)     -> &FieldData { &self.e_shnum }
    pub fn e_shstrndx(&self)  -> &FieldData { &self.e_shstrndx }
}


const EHDR_32_OFFSETS: &EhdrOffsets = &EhdrOffsets {
    e_type:      FieldData {offset: 16, length: 2},
    e_entry:     FieldData {offset: 24, length: 4},
    e_phoff:     FieldData {offset: 28, length: 4},
    e_shoff:     FieldData {offset: 32, length: 4},
    e_phentsize: FieldData {offset: 42, length: 2},
    e_phnum:     FieldData {offset: 44, length: 2},
    e_shentsize: FieldData {offset: 46, length: 2},
    e_shnum:     FieldData {offset: 48, length: 2},
    e_shstrndx:  FieldData {offset: 50, length: 2}
};

const EHDR_64_OFFSETS: &EhdrOffsets = &EhdrOffsets {
    e_type:      FieldData {offset: 16, length: 2},
    e_entry:     FieldData {offset: 24, length: 8},
    e_phoff:     FieldData {offset: 32, length: 8},
    e_shoff:     FieldData {offset: 40, length: 8},
    e_phentsize: FieldData {offset: 54, length: 2},
    e_phnum:     FieldData {offset: 56, length: 2},
    e_shentsize: FieldData {offset: 58, length: 2},
    e_shnum:     FieldData {offset: 60, length: 2},
    e_shstrndx:  FieldData {offset: 62, length: 2}
};


#[derive(Clone)]
pub struct Ehdr<L: Leaker> {
    leaker: Rc<L>,
    base_address: u64,
    ei_class:    RefCell<Option<Class>>,
    endian:      RefCell<Option<Endian>>,
    e_type:      RefCell<Option<u64>>,
    e_entry:     RefCell<Option<u64>>,
    e_phoff:     RefCell<Option<u64>>,
    e_phentsize: RefCell<Option<u64>>,
    e_phnum:     RefCell<Option<u64>>,
    e_shoff:     RefCell<Option<u64>>,
    e_shentsize: RefCell<Option<u64>>,
    e_shnum:     RefCell<Option<u64>>,
    e_shstrndx:  RefCell<Option<u64>>,
}


impl<L: Leaker> Ehdr<L> {
    pub fn new(address: u64, leaker: Rc<L>) -> Result<Ehdr<L>> {

        Ok(Ehdr {
            leaker: leaker,
            base_address: address,
            ei_class:    RefCell::new(None),
            endian:      RefCell::new(None),
            e_type:      RefCell::new(None),
            e_entry:     RefCell::new(None),
            e_phoff:     RefCell::new(None),
            e_phentsize: RefCell::new(None),
            e_phnum:     RefCell::new(None),
            e_shoff:     RefCell::new(None),
            e_shentsize: RefCell::new(None),
            e_shnum:     RefCell::new(None),
            e_shstrndx:  RefCell::new(None)
        })
    }

    pub fn ehdr_offsets(&self) -> Result<&'static EhdrOffsets> {
        Ok(match self.class()? {
            Class::Class32 => EHDR_32_OFFSETS,
            Class::Class64 => EHDR_64_OFFSETS,
        })
    }

    pub fn leaker(&self) -> &L { &self.leaker }
    pub fn base_address(&self) -> u64 { self.base_address }

    pub fn class(&self) -> Result<Class> {
        if self.ei_class.borrow().is_none() {
            let address = self.base_address() + 4;
            let ei_class =
                match self.leaker().leak_u8(address)? {
                    1 => Class::Class32,
                    2 => Class::Class64,
                    _ => bail!("Invalid Elf Class")
                };
            self.ei_class.replace(Some(ei_class));
        }
        Ok(self.ei_class.borrow().as_ref().unwrap().clone())
    }

    pub fn endian(&self) -> Result<Endian> {
        if self.endian.borrow().is_none() {
            let address = self.base_address() + 5;
            let endian =
                match self.leaker().leak_u8(address)? {
                    1 => Endian::Little,
                    2 => Endian::Big,
                    _ => bail!("Invalid Elf Class")
                };
            self.endian.replace(Some(endian));
        }
        Ok(self.endian.borrow().as_ref().unwrap().clone())
    }

    pub fn e_type(&self) -> Result<u64> {
        if self.e_type.borrow().is_none() {
            let base_address = self.base_address();
            let e_type = self.ehdr_offsets()?.e_type();
            let e_type = e_type.leak(base_address, self.leaker())?;
            self.e_type.replace(Some(e_type));
        }
        Ok(self.e_type.borrow().unwrap())
    }

    pub fn e_entry(&self) -> Result<u64> {
        if self.e_entry.borrow().is_none() {
            let base_address = self.base_address();
            let e_entry = self.ehdr_offsets()?.e_entry();
            let e_entry = e_entry.leak(base_address, self.leaker())?;
            self.e_entry.replace(Some(e_entry));
        }
        Ok(self.e_entry.borrow().unwrap())
    }

    pub fn e_phoff(&self) -> Result<u64> {
        if self.e_phoff.borrow().is_none() {
            let base_address = self.base_address();
            let e_phoff = self.ehdr_offsets()?.e_phoff();
            let e_phoff = e_phoff.leak(base_address, self.leaker())?;
            self.e_phoff.replace(Some(e_phoff));
        }
        Ok(self.e_phoff.borrow().unwrap())
    }

    pub fn e_shoff(&self) -> Result<u64> {
        if self.e_shoff.borrow().is_none() {
            let base_address = self.base_address();
            let e_shoff = self.ehdr_offsets()?.e_shoff();
            let e_shoff = e_shoff.leak(base_address, self.leaker())?;
            self.e_shoff.replace(Some(e_shoff));
        }
        Ok(self.e_shoff.borrow().unwrap())
    }

    pub fn e_phentsize(&self) -> Result<u64> {
        if self.e_phentsize.borrow().is_none() {
            let base_address = self.base_address();
            let e_phentsize = self.ehdr_offsets()?.e_phentsize();
            let e_phentsize = e_phentsize.leak(base_address, self.leaker())?;
            self.e_phentsize.replace(Some(e_phentsize));
        }
        Ok(self.e_phentsize.borrow().unwrap())
    }

    pub fn e_shentsize(&self) -> Result<u64> {
        if self.e_shentsize.borrow().is_none() {
            let base_address = self.base_address();
            let e_shentsize = self.ehdr_offsets()?.e_shentsize();
            let e_shentsize = e_shentsize.leak(base_address, self.leaker())?;
            self.e_shentsize.replace(Some(e_shentsize));
        }
        Ok(self.e_shentsize.borrow().unwrap())
    }

    pub fn e_phnum(&self) -> Result<u64> {
        if self.e_phnum.borrow().is_none() {
            let base_address = self.base_address();
            let e_phnum = self.ehdr_offsets()?.e_phnum();
            let e_phnum = e_phnum.leak(base_address, self.leaker())?;
            self.e_phnum.replace(Some(e_phnum));
        }
        Ok(self.e_phnum.borrow().unwrap())
    }

    pub fn e_shnum(&self) -> Result<u64> {
        if self.e_shnum.borrow().is_none() {
            let base_address = self.base_address();
            let e_shnum = self.ehdr_offsets()?.e_shnum();
            let e_shnum = e_shnum.leak(base_address, self.leaker())?;
            self.e_shnum.replace(Some(e_shnum));
        }
        Ok(self.e_shnum.borrow().unwrap())
    }

    pub fn e_shstrndx(&mut self) -> Result<u64> {
        if self.e_shstrndx.borrow().is_none() {
            let base_address = self.base_address();
            let e_shstrndx = self.ehdr_offsets()?.e_shstrndx();
            let e_shstrndx = e_shstrndx.leak(base_address, self.leaker())?;
            self.e_shstrndx.replace(Some(e_shstrndx));
        }
        Ok(self.e_shstrndx.borrow().unwrap())
    }
}