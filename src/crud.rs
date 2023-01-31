use colored::Colorize;
use rusqlite::{Connection, Result};

#[allow(dead_code)]
#[derive(Debug)]
struct Todo {
    id: u32,
    task: String,
    done: bool,
}

pub struct Crud {
    pub conn: Connection,
}

impl Crud {
    // Creates the database
    pub fn new(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE if not exists todo (
                id    INTEGER PRIMARY KEY,
                task  TEXT NOT NULL,
                done  INTEGER
            )",
            (),
        )?;
        Ok(())
    }

    // add new todo
    pub fn add(&self, todo: &str) -> Result<()> {
        let tobeadded = Todo { id: 0, task: todo.to_string(), done: false };
        self.conn.execute(
            "INSERT INTO todo (task, done) VALUES (?1, ?2)",
            (&tobeadded.task, &tobeadded.done),
        )?;
        Ok(())
    }

    // prints all todo
    pub fn show(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, task, done FROM todo")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo { id: row.get(0)?, task: row.get(1)?, done: row.get(2)? })
        })?;

        for todo in todo_iter {
            let t = todo.as_ref().unwrap();
            if !t.done {
                println!(
                    "[ ] {}. {}",
                    (t.id).to_string().green(),
                    (t.task).to_string().green()
                );
            } else {
                let formatted = format!(
                    "{}. {}",
                    (t.id).to_string().red(),
                    (t.task).to_string().red()
                );
                let formatted = formatted.dimmed().strikethrough();
                println!("[{}] {}", "X".bright_red(), formatted);
            }
        }
        Ok(())
    }

    // remove a existing todo
    pub fn delete(&self, index: &u32) -> Result<()> {
        self.conn
            .execute("DELETE FROM todo WHERE id = ?1;", [index.to_string()])?;
        Ok(())
    }

    // To delete all todo from the database
    pub fn drop(&self) -> Result<()> {
        self.conn.execute("DROP TABLE todo;", ())?;
        Ok(())
    }

    // Modify an existing todo
    pub fn modify(&self, index: &u32, new: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE todo SET task = ?1 WHERE id = ?2;",
            &[new, &index.to_string()],
        )?;
        Ok(())
    }

    // Ticks off a todo
    pub fn done(&self, index: &u32) -> Result<()> {
        self.conn.execute(
            "UPDATE todo SET done = 1 WHERE id = ?1;",
            [index.to_string()],
        )?;
        Ok(())
    }

    // show incomplete task
    pub fn incomplete(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, task, done FROM todo WHERE done = 0;")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo { id: row.get(0)?, task: row.get(1)?, done: row.get(2)? })
        })?;

        for todo in todo_iter {
            let t = todo.as_ref().unwrap();
            println!(
                "{}. {}",
                (t.id).to_string().green(),
                (t.task).to_string().green()
            );
        }
        Ok(())
    }

    pub fn complete(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, task, done FROM todo WHERE done = 1;")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo { id: row.get(0)?, task: row.get(1)?, done: row.get(2)? })
        })?;

        for todo in todo_iter {
            let t = todo.as_ref().unwrap();
            println!(
                "{}. {}",
                (t.id).to_string().red(),
                (t.task).to_string().red()
            );
        }
        Ok(())
    }
}
