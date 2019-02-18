use crate::serialize::Serialize;

pub struct Todo {
    list: String,
    done: bool,
    description: String,
}

impl Serialize for Todo {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<Todo> {
        match (
            String::deserialize(iter),
            bool::deserialize(iter),
            String::deserialize(iter),
        ) {
            (Some(list), Some(done), Some(description)) => Some(Todo {
                list,
                done,
                description,
            }),
            _ => None,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut v = self.list.serialize();
        v.append(&mut self.done.serialize());
        v.append(&mut self.description.serialize());
        v
    }
}
