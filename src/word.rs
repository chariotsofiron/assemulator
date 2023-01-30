pub trait UInt:
    From<u8>
    + TryFrom<u64>
    + std::fmt::Display
    + std::fmt::Debug
    + Clone
    + Copy
    + std::ops::BitOr<Output = Self>
{
}

impl UInt for usize {}
impl UInt for u8 {}
impl UInt for u16 {}
impl UInt for u32 {}
impl UInt for u64 {}
