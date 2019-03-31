extern crate jvec;

use jvec::JVec;

fn main() {
    let mut vec = JVec::new();
    vec[0] = Some('a');
    vec[1] = Some('b');
    vec[2] = Some('c');
    // some location ahead
    vec[10] = Some('d');

    // make a string out of it and for the undefined indices
    // use a space
    let string: String = vec.into_iter().map(|x| x.unwrap_or(' ')).collect();

    println!("{}", string);
}