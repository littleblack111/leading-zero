pub trait LeadingZeroCount {
    fn count_leading_zeros(&self) -> usize;
}

// ints

macro_rules! impl_for_ints {
    ($($t:ty),* $(,)?) => {
        $(
            impl LeadingZeroCount for $t {
                fn count_leading_zeros(&self) -> usize {
                    <$t>::leading_zeros(*self) as usize
                }
            }
        )*
    };
}

impl_for_ints!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,
);

// text/string

impl LeadingZeroCount for str {
    fn count_leading_zeros(&self) -> usize {
        let bytes = self.as_bytes();
        let mut i = 0;

        while i < bytes.len() && bytes[i] == b'0' {
            i += 1;
        }

        i
    }
}

impl LeadingZeroCount for String {
    fn count_leading_zeros(&self) -> usize {
        self.as_str()
            .count_leading_zeros()
    }
}

// bytes

impl LeadingZeroCount for [u8] {
    fn count_leading_zeros(&self) -> usize {
        let mut i = 0;

        while i < self.len() && self[i] == b'0' {
            i += 1;
        }

        i
    }
}

impl<const N: usize> LeadingZeroCount for [u8; N] {
    fn count_leading_zeros(&self) -> usize {
        self.as_slice()
            .count_leading_zeros()
    }
}

impl LeadingZeroCount for Vec<u8> {
    fn count_leading_zeros(&self) -> usize {
        self.as_slice()
            .count_leading_zeros()
    }
}

// char buffer

impl LeadingZeroCount for [char] {
    fn count_leading_zeros(&self) -> usize {
        let mut i = 0;

        while i < self.len() && self[i] == '0' {
            i += 1;
        }

        i
    }
}

impl<const N: usize> LeadingZeroCount for [char; N] {
    fn count_leading_zeros(&self) -> usize {
        self.as_slice()
            .count_leading_zeros()
    }
}

impl LeadingZeroCount for Vec<char> {
    fn count_leading_zeros(&self) -> usize {
        self.as_slice()
            .count_leading_zeros()
    }
}

// smart pointer and references

impl<T: LeadingZeroCount + ?Sized> LeadingZeroCount for &T {
    fn count_leading_zeros(&self) -> usize {
        (*self).count_leading_zeros()
    }
}

impl<T: LeadingZeroCount + ?Sized> LeadingZeroCount for &mut T {
    fn count_leading_zeros(&self) -> usize {
        (**self).count_leading_zeros()
    }
}

use std::borrow::Cow;

impl<'a> LeadingZeroCount for Cow<'a, str> {
    fn count_leading_zeros(&self) -> usize {
        self.as_ref()
            .count_leading_zeros()
    }
}

impl<'a> LeadingZeroCount for Cow<'a, [u8]> {
    fn count_leading_zeros(&self) -> usize {
        self.as_ref()
            .count_leading_zeros()
    }
}

use std::{rc::Rc, sync::Arc};

impl<T: LeadingZeroCount + ?Sized> LeadingZeroCount for Box<T> {
    fn count_leading_zeros(&self) -> usize {
        (**self).count_leading_zeros()
    }
}

impl<T: LeadingZeroCount + ?Sized> LeadingZeroCount for Rc<T> {
    fn count_leading_zeros(&self) -> usize {
        (**self).count_leading_zeros()
    }
}

impl<T: LeadingZeroCount + ?Sized> LeadingZeroCount for Arc<T> {
    fn count_leading_zeros(&self) -> usize {
        (**self).count_leading_zeros()
    }
}

// tests

#[cfg(test)]
mod tests {
    use super::LeadingZeroCount;
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    #[test]
    fn ints_bit_level() {
        assert_eq!(
            0u8.count_leading_zeros(),
            8
        );
        assert_eq!(
            1u8.count_leading_zeros(),
            7
        );
        assert_eq!(
            0u32.count_leading_zeros(),
            32
        );
        assert_eq!(
            1u32.count_leading_zeros(),
            31
        );
    }

    #[test]
    fn str_string() {
        assert_eq!(
            "".count_leading_zeros(),
            0
        );
        assert_eq!(
            "0".count_leading_zeros(),
            1
        );
        assert_eq!(
            "000123".count_leading_zeros(),
            3
        );
        assert_eq!(
            "123".count_leading_zeros(),
            0
        );

        let s = String::from("00042");
        assert_eq!(
            s.count_leading_zeros(),
            3
        );
    }

    #[test]
    fn bytes_vec_array() {
        let b: &[u8] = b"000abc";
        assert_eq!(
            b.count_leading_zeros(),
            3
        );

        let v = b"0012".to_vec();
        assert_eq!(
            v.count_leading_zeros(),
            2
        );

        let a: [u8; 4] = *b"000x";
        assert_eq!(
            a.count_leading_zeros(),
            3
        );
    }

    #[test]
    fn chars_vec_array() {
        let v: Vec<char> = "0009x"
            .chars()
            .collect();
        assert_eq!(
            v.count_leading_zeros(),
            3
        );

        let a: [char; 4] = [
            '0', '0', '1', 'x',
        ];
        assert_eq!(
            a.count_leading_zeros(),
            2
        );

        let slice: &[char] = &a;
        assert_eq!(
            slice.count_leading_zeros(),
            2
        );
    }

    #[test]
    fn smart_pointers_and_wrappers() {
        let s = String::from("0007");
        let b = Box::new(s.clone());
        let rc = Rc::new(s.clone());
        let arc = Arc::new(s.clone());

        assert_eq!(
            b.count_leading_zeros(),
            3
        );
        assert_eq!(
            rc.count_leading_zeros(),
            3
        );
        assert_eq!(
            arc.count_leading_zeros(),
            3
        );

        let cow_str: Cow<'_, str> = Cow::Owned(String::from("000x"));
        assert_eq!(
            cow_str.count_leading_zeros(),
            3
        );

        let cow_bytes: Cow<'_, [u8]> = Cow::Owned(b"001x".to_vec());
        assert_eq!(
            cow_bytes.count_leading_zeros(),
            2
        );
    }
}
