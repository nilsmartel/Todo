pub trait Serialize {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<Self>
    where
        Self: Sized;

    fn serialze(&self) -> Vec<u8>;
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

    fn serialze(&self) -> Vec<u8> {
        let n = *self;
        let m = |x| (x & 0xFF) as u8;
        vec![m(n >> 24), m(n >> 16), m(n >> 8), m(n)]
    }
}

impl Serialize for String {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<u64> {
        if let (Some(size) = u64::deserialize(iter) {
        
        }

        None
    }

    fn serialze(&self) -> Vec<u8> {}
}
