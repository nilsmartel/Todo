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
}

pub trait Serialize {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<Self>
    where
        Self: Sized;

    fn serialize(&self) -> Vec<u8>;
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

impl Serialize for u64 {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<u64> {
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => {
                Some((a as u64) << 24 | (b as u64) << 16 | (c as u64) << 8 | d as u64)
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
