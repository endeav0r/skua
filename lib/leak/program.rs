use error::*;
use goblin;
use leak::{ElfLeak, Leaker};
use leak::LinkMap;
use std::rc::Rc;
use std::collections::HashMap;



pub struct Program<L: Leaker> {
    leaker: Rc<L>,
    elfs: HashMap<String, ElfLeak<L>>
}


impl<L: Leaker> Program<L> {
    pub fn new(address: u64, leaker: Rc<L>) -> Result<Program<L>> {
        // get an elf
        let elf = ElfLeak::new(address, leaker.clone())?;

        let class = elf.ehdr()?.class()?.clone();

        // get the debug dynamic
        let dt_debug =
            elf.find_dynamic(goblin::elf::dyn::DT_DEBUG)?
                .ok_or("Failed to find DT_DEBUG")?;

        // get the link map
        let link_map_address = leaker.leak_u64(dt_debug.d_val()? + 8)?;
        let mut link_map = LinkMap::new(link_map_address, class, leaker.clone());

        // walk to the first link map
        while link_map.l_prev()? > 0 {
            link_map =
                LinkMap::new(
                    link_map.l_prev()?,
                    link_map.class().clone(),
                    leaker.clone());
        }

        // add elfs for every entry in the link map
        let mut elfs: HashMap<String, ElfLeak<L>> = HashMap::new();

        loop {
            elfs.insert(
                leaker.leak_string(link_map.l_name()?)?,
                ElfLeak::new(link_map.l_addr()?, leaker.clone())?);
            if link_map.l_next()? == 0 {
                break;
            }
            else {
                link_map = 
                    LinkMap::new(
                        link_map.l_next()?,
                        link_map.class().clone(),
                        leaker.clone());
            }
        }


        Ok(Program {
            leaker: leaker,
            elfs: elfs
        })
    }

    pub fn leaker(&self) -> &impl Leaker { self.leaker.as_ref() }
    pub fn elfs(&self) -> &HashMap<String, ElfLeak<L>> { &self.elfs }

    pub fn resolve(&self, lib_substring: &str, symbol: &str)
        -> Result<Option<u64>> {

        for (lib_name, elf) in self.elfs() {
            if !lib_name.contains(lib_substring) { continue; }

            let strtab =
                elf.find_dynamic(goblin::elf::dyn::DT_STRTAB)?
                    .ok_or("Failed to find DT_STRTAB")?
                    .d_val()?;

            for sym in elf.dynsyms()? {
                let name = sym.name(strtab)?;
                println!("0x{:08x}: {}", sym.st_value()?, name);
                if name == symbol {
                    return Ok(Some(sym.st_value()? + elf.base_address()));
                }
            }
        }

        Ok(None)
    }

}