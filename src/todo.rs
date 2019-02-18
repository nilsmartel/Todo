use crate::serialize::Serialize;

pub struct Todo {
    list: String,
    done: bool,
    description: String,
}

impl Serialize for Todo {}
