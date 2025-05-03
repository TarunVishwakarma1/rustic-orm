use rustic::HelloWorld;
use rustic_core::HelloWorld;

#[derive(HelloWorld)]
struct MyStruct {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_macro(){
        assert_eq!("Hello World! from MyStruct", MyStruct::hello_world());
    }
}