mod persistence;
mod serialize;
mod todo;

fn main() {
    let todos = persistence::load::<Vec<todo::Todo>>("todos").unwrap_or(Vec::new());
}
