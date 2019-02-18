use crate::serialize::Serialize;
use std::io::{Read, Write};
const PATH: &str = "~/.utils";

// TODO return Error instead of unwrapping
pub fn save<T>(item: T, key: &str)
where
    T: Serialize,
{
    if key == "" {
        panic!("variable key must be a valid file name");
    }

    if !std::path::Path::new(&format!("{}", PATH)).exists() {
        std::fs::create_dir(PATH).unwrap();
    }

    let mut file = std::fs::File::create(&format!("{}/{}", PATH, key)).unwrap();

    file.write_all(&item.serialize());
}

pub fn load<T>(key: &str) -> Option<T>
where
    T: Serialize,
{
    match std::fs::File::open(&format!("{}/{}", PATH, key)) {
        Ok(mut file) => {
            let mut buffer = Vec::<u8>::new();
            file.read_to_end(&mut buffer).unwrap();

            T::deserialize(&mut buffer.into_iter())
        }
        _ => None,
    }
}
