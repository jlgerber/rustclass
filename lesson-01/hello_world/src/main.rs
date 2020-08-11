mod ch01;
mod ch02;
mod ch03;

pub enum Chapters {
    Ch01,
    Ch02,
    Ch03,
}
fn main() {
    let current = Chapters::Ch03;

    match current {
        Chapters::Ch01 => ch01::main(),
        Chapters::Ch02 => ch02::main(),
        Chapters::Ch03 => ch03::main(),
    }
}
