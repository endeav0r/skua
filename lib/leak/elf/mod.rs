use error::*;
use goblin;
use leak::Leaker;
use std::cell::RefCell;
use std::rc::Rc;

mod dynamic;
mod ehdr;
mod link_map;
mod phdr;
mod shdr;
mod sym;

pub use self::dynamic::Dynamic;
pub use self::ehdr::Ehdr;
pub use self::link_map::LinkMap;
pub use self::phdr::Phdr;
pub use self::shdr::Shdr;
pub use self::sym::Sym;


#[derive(Clone, Debug)]
pub enum Class {
    Class32,
    Class64
}


#[derive(Clone, Debug)]
pub enum Endian {
    Big,
    Little
}


pub struct FieldData {
    offset: u64,
    length: usize // This is the length in bytes
}


impl FieldData {
    pub fn offset(&self) -> u64 { self.offset }
    pub fn length(&self) -> usize { self.length }

    pub fn leak(&self, base: u64, leaker: &impl Leaker) -> Result<u64> {
        let address = base + self.offset();
        match self.length {
            1 => leaker.leak_u8(address).map(|v| v as u64),
            2 => leaker.leak_u16(address).map(|v| v as u64),
            4 => leaker.leak_u32(address).map(|v| v as u64),
            8 => leaker.leak_u64(address),
            _ => bail!("Invalid length for leaking")
        }
    }
}


pub struct ElfLeak<L: Leaker> {
    leaker: Rc<L>,
    base_address: u64,
    ehdr: RefCell<Option<Rc<Ehdr<L>>>>,
    shdrs: RefCell<Option<Vec<Rc<Shdr<L>>>>>,
    phdrs: RefCell<Option<Vec<Rc<Phdr<L>>>>>,
    dynamics: RefCell<Option<Vec<Rc<Dynamic<L>>>>>,
    dynsyms: RefCell<Option<Vec<Rc<Sym<L>>>>>
}


impl<L: Leaker> ElfLeak<L> {
    pub fn new(mut address: u64, leaker: Rc<L>) -> Result<ElfLeak<L>> {
        // Find the beginning of the Elf
        address = address & 0xffffffff_fffff000;
        loop {
            if leaker.leak_buf(address, 1)?[0] == 0x7f && 
               leaker.leak_buf(address + 1, 1)?[0] == 0x45 && 
               leaker.leak_buf(address + 2, 1)?[0] == 0x4c && 
               leaker.leak_buf(address + 3, 1)?[0] == 0x46 {
                break;
            }
            address -= 0x1000;
        }

        Ok(ElfLeak {
            leaker: leaker,
            base_address: address,
            ehdr: RefCell::new(None),
            shdrs: RefCell::new(None),
            phdrs: RefCell::new(None),
            dynamics: RefCell::new(None),
            dynsyms: RefCell::new(None)
        })
    }

    pub fn leaker(&self) -> &L { &self.leaker }
    pub fn base_address(&self) -> u64 { self.base_address }

    pub fn address_base(&self) -> Result<u64> {
        match self.ehdr()?.e_type()? as u16 {
            goblin::elf::header::ET_REL |
            goblin::elf::header::ET_DYN => Ok(self.base_address()),
            goblin::elf::header::ET_EXEC => Ok(0),
            _ => bail!("Invalid ehdr.e_type={}", self.ehdr()?.e_type()?)
        }
    }

    pub fn ehdr(&self) -> Result<Rc<Ehdr<L>>> {
        if self.ehdr.borrow().is_none() {
            let ehdr = Ehdr::new(self.base_address(), self.leaker.clone())?;
            self.ehdr.replace(Some(Rc::new(ehdr)));
        }
        Ok(self.ehdr.borrow().as_ref().unwrap().clone())
    }

    pub fn shdrs(&self) -> Result<Vec<Rc<Shdr<L>>>> {
        if self.shdrs.borrow().is_none() {
            let offset = self.ehdr()?.e_shoff()?;
            let shentsize = self.ehdr()?.e_shentsize()?;
            let shnum = self.ehdr()?.e_shnum()?;
            let class = self.ehdr()?.class()?;
            let mut shdrs: Vec<Rc<Shdr<L>>> = Vec::new();
            for i in 0..shnum {
                let address = self.base_address() + offset + (i * shentsize);
                shdrs.push(
                    Rc::new(
                        Shdr::new(address, class.clone(), self.leaker.clone())));
            }
            self.shdrs.replace(Some(shdrs));
        }
        Ok(self.shdrs.borrow().as_ref().unwrap().clone())
    }

    pub fn phdrs(&self) -> Result<Vec<Rc<Phdr<L>>>> {
        if self.phdrs.borrow().is_none() {
            let offset = self.ehdr()?.e_phoff()?;
            let phentsize = self.ehdr()?.e_phentsize()?;
            let phnum = self.ehdr()?.e_phnum()?;
            let class = self.ehdr()?.class()?;
            let mut phdrs: Vec<Rc<Phdr<L>>> = Vec::new();
            for i in 0..phnum {
                let address = self.base_address() + offset + (i * phentsize);
                phdrs.push(
                    Rc::new(
                        Phdr::new(address, class.clone(), self.leaker.clone())));
            }
            self.phdrs.replace(Some(phdrs));
        }
        Ok(self.phdrs.borrow().as_ref().unwrap().clone())
    }

    pub fn dynamics(&self) -> Result<Vec<Rc<Dynamic<L>>>> {
        if self.dynamics.borrow().is_none() {
            let mut self_dynamics: Option<Vec<Rc<Dynamic<L>>>> = None;
            let class = self.ehdr()?.class()?;
            let leaker = self.leaker.clone();
            let address_base = self.address_base()?;
            for phdr in self.phdrs()? {
                if phdr.p_type()? as u32 == goblin::elf::program_header::PT_DYNAMIC {
                    let dynamic_size = match class {
                        Class::Class32 => 8,
                        Class::Class64 => 16
                    };
                    let mut dynamics = Vec::new();
                    for i in 0..(phdr.p_memsz()? / dynamic_size) {
                        let address =
                            address_base + phdr.p_vaddr()? + (i * dynamic_size);
                        let dynamic =
                            Rc::new(
                                Dynamic::new(
                                    address,
                                    class.clone(),
                                    leaker.clone()));
                        dynamics.push(dynamic);
                    }
                    self_dynamics = Some(dynamics);
                    break;
                }
            }
            self.dynamics.replace(self_dynamics);
        }
        Ok(self.dynamics.borrow().as_ref().unwrap().clone())
    }

    pub fn find_dynamic(&self, d_tag: u64) -> Result<Option<Rc<Dynamic<L>>>> {
        for dynamic in self.dynamics()? {
            if dynamic.d_tag()? == d_tag {
                return Ok(Some(dynamic));
            }
        }
        Ok(None)
    }


    pub fn find_shdr(&self, type_: u64) -> Result<Option<Rc<Shdr<L>>>> {
        for shdr in self.shdrs()? {
            println!("checking shdr, {}", shdr.sh_type()?);
            if shdr.sh_type()? == type_ {
                return Ok(Some(shdr));
            }
        }
        Ok(None)
    }

    pub fn dynsyms(&self) -> Result<Vec<Rc<Sym<L>>>> {
        if self.dynsyms.borrow().is_none() {

            // find the number of dynamic symbols by fetching the nchain value
            // from DT_HASH
            let dt_hash =
                self.find_dynamic(goblin::elf::dyn::DT_HASH as u64)?
                    .ok_or("Failed to find DT_HASH")?;

            println!("dt_hash: 0x{:x}", dt_hash.d_val()?);

            // let nbucket = self.leaker().leak_u32(dt_hash.d_val()?)?;
            let nchain = self.leaker().leak_u32(dt_hash.d_val()? + 4)?;

            // nchain is the number of dynamic symbols

            let dynsym_base_address =
                self.find_dynamic(goblin::elf::dyn::DT_SYMTAB)?
                    .ok_or("Failed to find DT_SYMTAB")?
                    .d_val()?;

            // let strtab_base_address =
            //     self.find_dynamic(goblin::elf::dyn::DT_STRTAB)?
            //         .ok_or("Failed to find DT_STRTAB")?
            //         .d_val()?;

            let dt_syment =
                self.find_dynamic(goblin::elf::dyn::DT_SYMENT)?
                    .ok_or("Failed to find DT_SYMENT")?
                    .d_val()?;

            let mut dynsyms = Vec::new();

            for i in 0..(nchain as u64) {
                let address = dynsym_base_address + dt_syment * i;
                let sym = Sym::new(
                    address,
                    self.ehdr()?.class()?.clone(),
                    self.leaker.clone());
                dynsyms.push(Rc::new(sym));
            }

            self.dynsyms.replace(Some(dynsyms));
        }


        Ok(self.dynsyms.borrow().as_ref().unwrap().clone())
    }
}