use std::{fmt, ops};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct BitMap8(u8);

impl BitMap8 {
    pub fn new(value: u8) -> BitMap8 {
        BitMap8(value)
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for BitMap8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::Debug for BitMap8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bitmap8({:X})", self.0)
    }
}

impl ops::BitAnd for BitMap8 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ops::BitOr for BitMap8 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ops::BitXor for BitMap8 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ops::BitAndAssign for BitMap8 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ops::BitOrAssign for BitMap8 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ops::BitXorAssign for BitMap8 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ops::Deref for BitMap8 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::bitmap8::BitMap8;

    #[test]
    fn to_u8_yields_u8() {
        let new_bitmap = BitMap8::new(1 << 2);
        let corresponding_u8 = 1 << 2;
        assert_eq!(new_bitmap.to_u8(), corresponding_u8);
    }

    #[test]
    fn impl_display_prints_u8() {
        let new_bitmap = BitMap8::new(1 << 2);
        let corresponding_u8_as_string = format!("{:b}", 1 << 2);
        assert_eq!(new_bitmap.to_string(), corresponding_u8_as_string);
    }

    #[test]
    fn impl_debug_pretty_prints_bitmap() {
        let new_bitmap = BitMap8::new(1 << 2);
        let shows_as = "Bitmap8(4)";
        assert_eq!(format!("{:#?}", new_bitmap), shows_as);
    }

    #[test]
    fn impl_basic_bit_ops_works_as_u8s() {
        let result_of_bitmap_and = BitMap8::new(1 << 0) & BitMap8::new(1 << 1);
        let result_of_bitmap_xor = BitMap8::new(1 << 0) ^ BitMap8::new(1 << 1);
        let result_of_bitmap_or = BitMap8::new(1 << 0) | BitMap8::new(1 << 1);

        let result_of_and = BitMap8::new((1 << 0) & (1 << 1));
        let result_of_xor = BitMap8::new((1 << 0) ^ (1 << 1));
        let result_of_or = BitMap8::new((1 << 0) | (1 << 1));

        assert_eq!(result_of_bitmap_and, result_of_and);
        assert_eq!(result_of_bitmap_xor, result_of_xor);
        assert_eq!(result_of_bitmap_or, result_of_or);
    }

    #[test]
    fn impl_assign_bit_ops_works_as_u8s() {
        let mut base_bitmap = BitMap8::new(0);
        let mut base_u8 = 0u8;

        base_bitmap |= BitMap8::new(1 << 0);
        base_u8 |= 1 << 0;
        assert_eq!(base_bitmap.to_u8(), base_u8);

        base_bitmap &= BitMap8::new(1 << 0);
        base_u8 &= 1 << 0;
        assert_eq!(base_bitmap.to_u8(), base_u8);

        base_bitmap ^= BitMap8::new(1 << 0);
        base_u8 ^= 1 << 0;
        assert_eq!(base_bitmap.to_u8(), base_u8);
    }

    #[test]
    fn impl_deref_yields_u8() {
        let underlying_u8 = 1 << 1;
        let bitmap = BitMap8::new(underlying_u8);
        assert_eq!(*bitmap, underlying_u8);
    }
}
