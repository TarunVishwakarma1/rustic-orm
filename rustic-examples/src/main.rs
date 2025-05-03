use rustic_core::HelloWorld;
use rustic::HelloWorld;

#[derive(HelloWorld)]
pub struct MyStruct {
}
fn main() {
    MyStruct::hello_world();
}

