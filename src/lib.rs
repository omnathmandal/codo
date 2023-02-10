mod cli;
mod crud;
pub mod interface;

#[cfg(test)]
mod tests {

    #[test]
    fn internals() {
        use crate::crud::Crud;
        use dotenv::dotenv;
        use rusqlite::Connection;
        use std::{env, path::Path};

        dotenv().ok();
        let url = env::var("DATABASE_URL").unwrap();
        let connection = Connection::open(url).unwrap();

        let mut comp = Vec::new();
        let mut notdone = Vec::new();

        let mut todo =
            Crud { conn: connection, com: &mut comp, inc: &mut notdone };

        todo.new().unwrap();

        let path = Path::new("todo.db");
        assert!(path.exists());
    }
}
