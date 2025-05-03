use rustic::{HelloWorld, SqlModel};
use rustic_core::HelloWorld;

#[derive(HelloWorld, SqlModel)]
struct User {
    id: i32,
    name: String,
    age: u32, 
    email: String,
    active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_macro(){
        assert_eq!("Hello World! from User", User::hello_world());
    }

    #[test]
    fn create_table_sql_macro(){
        assert_eq!("CREATE TABLE IF NOT EXISTS user (id INTEGER, name TEXT, age INTEGER, email TEXT, active BOOLEAN);", User::create_table_sql());
    }

    #[test]
    fn select_all_sql_macro(){
        assert_eq!("SELECT * FROM user;", User::select_all_sql());
    }

    #[test]
    fn select_by_id_sql_macro(){
        assert_eq!("SELECT * FROM user WHERE id = '1';", User::select_by_id_sql("id", "1"));
    }

    #[test]
    fn insert_sql_macro(){
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 30,
            active: true,
        };
        assert_eq!("INSERT INTO user (id, name, age, email, active) VALUES ('1', 'Alice', '30', 'alice@example.com', 'true');", user.insert_sql())
    }

    #[test]
    fn update_sql_macro(){
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 30,
            active: true,
        };
        assert_eq!("UPDATE user SET id = '1', name = 'Alice', age = '30', email = 'alice@example.com', active = 'true' WHERE id = '1';", user.update_sql("id", "1"));
    }

    #[test]
    fn delete_sql_macro(){
        assert_eq!("DELETE FROM user WHERE id = '1';", User::delete_sql("id", "1"));
    }
}