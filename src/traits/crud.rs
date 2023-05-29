// use async_trait::async_trait;
// use serde::{Deserialize, Serialize};
// use sqlx::{Database, FromRow, SqlitePool};
//

use std::println;

trait CrudOps<T> {
    fn create<T>(&self, data: T) -> Result<(), String>;
    fn read<T>(&self, id: i32) -> Result<T, String>;
}

pub struct User {
    id: i32,
}
struct Movie {
    id: i32,
}

struct Crud<T> {
    data: T,
}
impl<T> CrudOps<T> for Crud<T> {
    fn read<T>(&self, id: i32) -> Result<T, String> {
        let query = format!("insert into database");
        println!("id {id}");
        println!("T:{}");
        Ok(T)
    }
}

fn mm() {
    let crud = Crud::<User> {
        data: User { id: 4 },
    };

    crud.read(data);
}

     let h = 3;

mm();
