extern crate byteorder;
#[macro_use] extern crate error_chain;
extern crate goblin;

pub mod leak;
mod stream;

pub use self::stream::Stream;


pub mod error {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }

        foreign_links {
            FromUtf8Error(::std::string::FromUtf8Error);
            IoError(::std::io::Error);
        }

        errors {
            LeakFail(address: u64) {
                description("Failed to leak bytes from address")
                display("Failed to leak bytes from {:x}", address)
            }
        }
    }
}