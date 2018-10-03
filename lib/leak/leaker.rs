use error::*;
use leak::Endian;

pub trait Leaker: Clone {
    fn leak_u8(&self, address: u64) -> Result<u8>;
    fn endian(&self) -> &Endian;

    fn leak_u16(&self, address: u64) -> Result<u16> {
        match self.endian() {
            Endian::Big => {
                let hi = self.leak_u8(address + 0)? as u16;
                let lo = self.leak_u8(address + 1)? as u16;
                Ok((hi << 8) | lo)
            },
            Endian::Little => {
                let hi = self.leak_u8(address + 1)? as u16;
                let lo = self.leak_u8(address + 0)? as u16;
                Ok((hi << 8) | lo)
            }
        }
    }

    fn leak_u32(&self, address: u64) -> Result<u32> {
        let bytes = self.leak_buf(address, 4)?;
        match self.endian() {
            Endian::Big => {
                let mut value: u32 = 0;
                for i in 0..4 {
                    let byte: u32 = bytes[i] as u32;
                    value = value | (byte << (8 * (3 - i)));
                }
                Ok(value)
            },
            Endian::Little => {
                let mut value: u32 = 0;
                for i in 0..4 {
                    let byte: u32 = bytes[i] as u32;
                    value = value | (byte << (8 * (i)));
                }
                Ok(value)
            }
        }
    }

    fn leak_u64(&self, address: u64) -> Result<u64> {
        let bytes = self.leak_buf(address, 8)?;
        match self.endian() {
            Endian::Big => {
                let mut value: u64 = 0;
                for i in 0..8 {
                    let byte: u64 = bytes[i] as u64;
                    value = value | (byte << (8 * (7 - i)));
                }
                Ok(value)
            },
            Endian::Little => {
                let mut value: u64 = 0;
                for i in 0..8 {
                    let byte: u64 = bytes[i] as u64;
                    value = value | (byte << (8 * (i)));
                }
                Ok(value)
            }
        }
    }

    fn leak_buf(&self, address: u64, length: usize) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        for i in 0..length {
            bytes.push(self.leak_u8(address + i as u64)?);
        }
        Ok(bytes)
    }

    fn leak_string(&self, address: u64) -> Result<String> {
        let mut bytes = Vec::new();
        let mut offset: u64 = 0;
        loop {
            let byte = self.leak_u8(address + offset)?;
            if byte == 0 {
                break;
            }
            bytes.push(byte);
            offset += 1;
        }
        Ok(String::from_utf8(bytes)?)
    }
}