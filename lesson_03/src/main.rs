mod foo;
use foo::bla;
mod widgets;
use widgets::cube::mybox;

fn main() {
    println!("{}", bla());
    other::ahh();
    mybox();
}

pub mod other {
    pub fn ahh() {
        println!("ahhh");
    }
}
