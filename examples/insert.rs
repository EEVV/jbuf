extern crate jvec;

use jvec::JVec;

fn main() {
    let mut vec = JVec::new();
    vec[0] = Some(5);
    vec.insert(0, 0);
    vec.insert(5, 42);
    vec.insert(0, 0);

    for i in vec {
        println!("{:?}", i);
    }
}