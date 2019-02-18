mod persistence;
mod serialize;
mod todo;

fn main() {
    let todo = persistence::load::<todo::Todo>("testing");

    dbg!(todo);
}
