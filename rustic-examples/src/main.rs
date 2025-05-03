use rustic_core::HelloWorld;
use rustic::{HelloWorld, SqlModel};

#[derive(HelloWorld)]
pub struct MyStruct {
}

#[derive(SqlModel)]
struct User {
    id: i32,
    name: String,
    email: String,
    age: i64,
}

fn main() {
    let s = MyStruct::hello_world();
    println!("{}",s);

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };

    // Ensure all methods are called correctly
    println!("Create Table Query: {}", User::create_table_sql());
    println!("Insert Query: {}", user.insert_sql());
    println!("Select All Query: {}", User::select_all_sql());
    println!("Select By ID Query: {}", User::select_by_id_sql("id", "1"));
    println!("Update Query: {}", user.update_sql("id", "1"));
    println!("Delete Query: {}", User::delete_sql("id", "1"));
}

