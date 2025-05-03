use rustic_core::HelloWorld;
use rustic::HelloWorld;

#[derive(HelloWorld)]
pub struct MyStruct {
}
fn main() {
    let s = MyStruct::hello_world();
    println!("{}",s);
}

