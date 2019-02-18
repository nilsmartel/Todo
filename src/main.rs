mod persistence;
mod serialize;
mod todo;

fn main() {
    let todos = read_todo_file();
}

fn read_todo_file() -> Vec<todo::Todo> {
    Vec::new()
}
