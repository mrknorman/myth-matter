//! Physical binding to a body, separate from phase and recipe content.
//!
//! V1 stores one byte: 0 = intact, 1 = loose. Unknown values refuse. A
//! transfer or persistence record carries this alongside the recipe key;
//! changing it does not change that recipe's canonical bytes or content key.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u8)]
pub enum MaterialBindingStateV1 {
    Intact = 0,
    Loose = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialBindingStateRefusalV1 {
    UnknownTag(u8),
}

impl MaterialBindingStateV1 {
    pub const fn to_byte(self) -> u8 {
        self as u8
    }

    pub const fn from_byte(tag: u8) -> Result<Self, MaterialBindingStateRefusalV1> {
        match tag {
            0 => Ok(Self::Intact),
            1 => Ok(Self::Loose),
            other => Err(MaterialBindingStateRefusalV1::UnknownTag(other)),
        }
    }
}
