use crate::serialize::Serialize;

#[derive(Debug)]
pub struct Todo {
    list: Alphanumeric,
    done: bool,
    description: String,
}

#[derive(Debug)]
pub struct Alphanumeric(String);

impl Alphanumeric {
    pub fn new() -> Alphanumeric {
        Alphanumeric(String::new())
    }
}

impl From<String> for Alphanumeric {
    fn from(item: String) -> Alphanumeric {
        Alphanumeric::from(item.as_str())
    }
}

impl From<&str> for Alphanumeric {
    fn from(item: &str) -> Alphanumeric {
        Alphanumeric(
            item.to_lowercase()
                .chars()
                .filter(|&c| c >= 'a' && c <= 'z' || c >= '0' && c <= '9')
                .collect(),
        )
    }
}

impl Serialize for Alphanumeric {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<Alphanumeric> {
        match String::deserialize(iter) {
            Some(s) => Some(Alphanumeric::from(s)),
            _ => None,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            list: Alphanumeric::new(),
            done: false,
            description: String::new(),
        }
    }

    pub fn with_list(mut self, list: Alphanumeric) -> Todo {
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

    pub fn set_list(&mut self, list: Alphanumeric) {
        self.list = list;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

impl Serialize for Todo {
    fn deserialize(iter: &mut Iterator<Item = u8>) -> Option<Todo> {
        match (
            Alphanumeric::deserialize(iter),
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
