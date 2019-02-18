#[cfg(test)]
mod tests {
    use crate::serialize::Serialize;

    #[test]
    fn test_string() {
        let s = String::from("Hello World, my name is Nils Martel😃");
        let serde = String::deserialize(&mut s.serialize().into_iter()).unwrap();

        assert_eq!(serde, s);
    }

    #[test]
    fn test_bool() {
        let bools = [false, true];

        for b in bools.iter() {
            let serde = bool::deserialize(&mut b.serialize().into_iter()).unwrap();
            assert_eq!(serde, *b);
        }
    }

    #[test]
    fn test_u64() {
        let numbers = [
            0u64,
            1,
            1 << 8,
            1 << 16,
            1 << 24,
            1 << 32,
            1 << 40,
            1 << 48,
            1 << 56,
            !0,
            2,
            10,
            12,
            300,
            42323,
            43243243,
            654753478,
            98795456765,
            2312464356,
        ];

        for n in numbers.iter() {
            let serde: u64 = u64::deserialize(&mut n.serialize().into_iter()).unwrap();
            assert_eq!(serde, *n);
        }
    }

    #[test]
    fn test_vec() {
        let numbers: Vec<u64> = vec![
            0,
            1,
            1 << 8,
            1 << 16,
            1 << 24,
            1 << 32,
            1 << 40,
            1 << 48,
            1 << 56,
            !0,
            2,
            10,
            12,
            300,
            42323,
            43243243,
            654753478,
            98795456765,
            2312464356,
        ];

        let serde = Vec::<u64>::deserialize(&mut numbers.serialize().into_iter()).unwrap();

        assert_eq!(serde, numbers);
    }
}

pub trait Serialize {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<Self>
    where
        Self: Sized;

    fn serialize(&self) -> Vec<u8>;
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<Self> {
        if let Some(size) = u64::deserialize(iter) {
            let mut v = Vec::with_capacity(size as usize);

            for _ in 0..size {
                match T::deserialize(iter) {
                    Some(elem) => v.push(elem),
                    _ => return None,
                }
            }

            return Some(v);
        }

        None
    }

    fn serialize(&self) -> Vec<u8> {
        let mut v = (self.len() as u64).serialize();
        v.extend(self.iter().flat_map(|elem| elem.serialize()));

        v
    }
}

impl Serialize for bool {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<bool> {
        match iter.next() {
            Some(x) => Some(x != 0),
            _ => None,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl Serialize for u32 {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<u32> {
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => {
                Some((a as u32) << 24 | (b as u32) << 16 | (c as u32) << 8 | d as u32)
            }
            _ => None,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let n = *self;
        let m = |x| (x & 0xFF) as u8;
        vec![m(n >> 24), m(n >> 16), m(n >> 8), m(n)]
    }
}

impl Serialize for u64 {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<u64> {
        match (u32::deserialize(iter), u32::deserialize(iter)) {
            (Some(a), Some(b)) => Some((a as u64) << 32 | b as u64),
            _ => None,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let n = *self;
        let m = |x| (x & 0xFF) as u8;

        vec![
            m(n >> 56),
            m(n >> 48),
            m(n >> 40),
            m(n >> 32),
            m(n >> 24),
            m(n >> 16),
            m(n >> 8),
            m(n),
        ]
    }
}

impl Serialize for String {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<String> {
        if let Some(size) = u64::deserialize(iter) {
            return match String::from_utf8(iter.take(size as usize).collect::<Vec<u8>>()) {
                Ok(s) => Some(s),
                _ => None,
            };
        }

        None
    }

    fn serialize(&self) -> Vec<u8> {
        let mut vec = (self.len() as u64).serialize();
        vec.append(&mut self.as_bytes().to_vec());
        vec
    }
}
