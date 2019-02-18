use crate::serialize::Serialize;

pub struct Todo {
    list: String,
    done: bool,
    description: String,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            list: String::new(),
            done: false,
            description: String::new(),
        }
    }

    pub fn with_list(mut self, list: String) -> Todo {
        self.list = list;
        self
    }

    pub fn with_description(mut self, description: String) -> Todo {
        self.description = description;
        self
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    pub fn set_list(&mut self, list: String) {
        self.list = list;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
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
