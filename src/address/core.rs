#[derive(Debug, PartialEq)]
pub struct Address {
    pub high_byte: u8,
    pub low_byte: u8
}

impl Address {
    pub fn from_bytes(addresses: [u8; 2]) -> Self {
        let high_byte = addresses[0];
        let low_byte = addresses[1];

        Self { high_byte: high_byte, low_byte: low_byte }
    }

    pub fn from_integer(address: u16) -> Self {
        let addresses = address.to_be_bytes();

        Self { high_byte: addresses[0], low_byte: addresses[1] }
    }

    pub fn as_bytes(&self) -> [u8; 2] {
        [self.high_byte, self.low_byte]
    }

    pub fn as_integer(&self) -> u16 {
        u16::from_be_bytes(self.as_bytes())
    }
}