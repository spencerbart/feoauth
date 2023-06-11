use crate::some_module::file_a::bla;

mod some_module;

fn main () {
    let bla = bla();
    println!("{}", bla);
}