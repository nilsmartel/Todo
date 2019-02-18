use crate::serialize::Serialize;

pub struct Todo {
    list: String,
    done: bool,
    description: String,
}

impl Serialize for Todo {
    fn description(iter: &mut Iterator<Item = u8>) -> Todo {}
}
