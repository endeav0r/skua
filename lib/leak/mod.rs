mod elf;
mod leaker;
mod program;

pub use self::leaker::Leaker;
pub use self::program::Program;
pub use self::elf::{Dynamic, Ehdr, ElfLeak, LinkMap, Phdr, Shdr, Sym};

pub enum Endian {
    Big,
    Little
}