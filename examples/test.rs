extern crate jvec;

use jvec::JVec;

fn main() {
    let mut vec = JVec::new();
    vec[0] = Some(0);
    vec[5] = Some(5);
    vec[10] = Some(10);

    for i in vec {
        println!("{:?}", i);
    }
}