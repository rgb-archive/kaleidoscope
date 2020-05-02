use core::convert::TryFrom;

/// Magic numbers here are used to distinguish files or bech32-encoded strings
/// with RGB data of different type.
///
/// NB: These numbers are used with binary data serialization only; they are
/// not a part of the commitments, ids or network-serialized packets.
///
/// Rationale: convenience for wallet data import/export and extra-wallet
/// file storage; additional check not to mis-interpret byte sequences
#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
#[display_from(Debug)]
#[repr(u32)]
pub enum MagicNumber {
    /// Equals to first 4 bytes of SHA26("rgb:schema")
    /// = 18429ce35af7f898f765417b28471ab454b89ceff6fc33de77ff5fd98e066bc3
    /// Check with `echo -n "rgb:schema" | shasum -a 256`
    Schema = 0x18429ce3,

    /// Equals to first 4 bytes of SHA26("rgb:gensis")
    /// = 2e91cbc08b6205efb4f908bb9bd3fcf5c148763f7b23b0506ef64ffd414fc9b4
    Genesis = 0x2e91cbc0,

    /// Equals to first 4 bytes of SHA26("rgb:transition")
    /// = bf11926e3db131632bdfa8f996d52d6d19e25d0884c922365ff8cd3c73f10198
    Transition = 0xbf11926e,

    /// Equals to first 4 bytes of SHA26("rgb:anchor")
    /// = dd53b6f17c16915ecd01de7935b5c38497f6f6c49b97627296496dc31a6ca86b
    Anchor = 0xdd53b6f1,

    /// Equals to first 4 bytes of SHA26("rgb:consignment")
    /// = 4c82bf5385ab9027f15f1ce17a8007956fe8f38cbad2ee312cf2c55b72a69420
    Consignment = 0x4c82bf53,

    /// Equals to first 4 bytes of SHA26("rgb:stash")
    /// = cd22a2cb85720d51f1616752cb85059a02f3d35f7dda30a4ca981b59b0924354
    Stash = 0xcd22a2cb,
}

impl MagicNumber {
    pub fn to_u32(&self) -> u32 {
        use std::mem;
        let m;
        unsafe {
            m = mem::transmute::<Self, u32>(self.clone());
        }
        m as u32
    }
}

impl TryFrom<u32> for MagicNumber {
    type Error = u32;
    fn try_from(number: u32) -> Result<Self, Self::Error> {
        Ok(match number {
            n if n == Self::Schema.to_u32() => Self::Schema,
            n if n == Self::Genesis.to_u32() => Self::Genesis,
            n if n == Self::Transition.to_u32() => Self::Transition,
            n if n == Self::Anchor.to_u32() => Self::Anchor,
            n if n == Self::Consignment.to_u32() => Self::Consignment,
            n if n == Self::Stash.to_u32() => Self::Stash,
            invalid => Err(invalid)?,
        })
    }
}
