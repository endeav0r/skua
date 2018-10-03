use byteorder::{LittleEndian, WriteBytesExt};
use error::*;
use std::cell::RefCell;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Read, Write};
use std::sync::Arc;


#[derive(Clone)]
pub struct Stream {
    stream: Arc<RefCell<TcpStream>>
}


impl Stream {
    pub fn connect<A: ToSocketAddrs>(address: A) -> Result<Stream> {
        Ok(Stream {
            stream: Arc::new(RefCell::new(TcpStream::connect(address)?))
        })
    }


    pub fn send(&self, data: &[u8]) -> Result<()> {
        self.stream.borrow_mut().write_all(data)?;
        Ok(())
    }


    pub fn send_le64(&self, data: u64) -> Result<()> {
        self.stream.borrow_mut().write_u64::<LittleEndian>(data)?;
        Ok(())
    }


    pub fn recv(&self) -> Result<Vec<u8>> {
        let mut buf: [u8; 1500] = [0; 1500];

        let bytes_read = self.stream.borrow_mut().read(&mut buf)?;

        let mut buf = buf.to_vec();
        buf.truncate(bytes_read);

        Ok(buf)
    }


    pub fn recv_byte(&self) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];

        self.stream.borrow_mut().read(&mut buf)?;

        Ok(buf[0])
    }


    pub fn recv_line(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        loop {
            let byte = self.recv_byte()?;
            if byte == 0xa { break; }
            bytes.push(byte);
        }

        Ok(bytes)
    }
}