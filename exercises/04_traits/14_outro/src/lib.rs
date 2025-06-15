// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Debug, Clone, Copy)] //, PartialEq, Eq, PartialOrd, Ord)]
pub struct SaturatingU16(u16);

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        SaturatingU16(value)
    }

    pub fn from_u8(value: u8) -> Self {
        SaturatingU16(value as u16)
    }

    pub fn from_ref(value: &u16) -> Self {
        SaturatingU16(*value)
    }

    pub fn from_ref_u8(value: &u8) -> Self {
        SaturatingU16(*value as u16)
    }

    pub fn saturating_add(self, other: SaturatingU16) -> Self {
        SaturatingU16(self.0.saturating_add(other.0))
    }

    pub fn saturating_add_u16(self, other: u16) -> Self {
        SaturatingU16(self.0.saturating_add(other))
    }

    pub fn saturating_add_ref(&self, other: &SaturatingU16) -> Self {
        SaturatingU16(self.0.saturating_add(other.0))
    }

    pub fn saturating_add_ref_u16(&self, other: &u16) -> Self {
        SaturatingU16(self.0.saturating_add(*other))
    }

    pub fn saturating_add_ref_u8(&self, other: &u8) -> Self {
        SaturatingU16(self.0.saturating_add(*other as u16))
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16(value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16(value as u16)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16(*value as u16)
    }
}

impl std::ops::Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.saturating_add(other)
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u16) -> Self::Output {
        self.saturating_add_u16(other)
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &SaturatingU16) -> Self::Output {
        self.saturating_add_ref(other)
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u16) -> Self::Output {
        self.saturating_add_ref_u16(other)
    }
}

impl std::ops::Add<&u8> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u8) -> Self::Output {
        self.saturating_add_ref_u8(other)
    }
}

impl std::cmp::PartialEq for SaturatingU16 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::cmp::PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

