use colored::Colorize;
use rusqlite::{Connection, Result};

#[allow(dead_code)]
#[derive(Debug)]
struct Todo {
    id: u32,
    task: String,
    done: bool,
}

pub struct Crud<'a> {
    pub conn: Connection,
    pub com: &'a mut Vec<u32>,
    pub inc: &'a mut Vec<u32>,
}

impl<'a> Crud<'_> {
    // Creates the database
    pub fn new(&mut self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE if not exists todo (
                id    INTEGER PRIMARY KEY,
                task  TEXT NOT NULL,
                done  INTEGER
            )",
            (),
        )?;
        let mut stmt = self.conn.prepare("SELECT id, task, done FROM todo")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo { id: row.get(0)?, task: row.get(1)?, done: row.get(2)? })
        })?;

        for todo in todo_iter {
            let t = todo.as_ref().unwrap();
            if !t.done {
                self.inc.push(t.id);
            } else {
                self.com.push(t.id);
            }
        }
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
        if self.inc.len() + self.com.len() != 0 {
            let mut stmt =
                self.conn.prepare("SELECT id, task, done FROM todo")?;
            let todo_iter = stmt.query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    task: row.get(1)?,
                    done: row.get(2)?,
                })
            })?;
            println!("{}", "TODO".bright_blue().underline());
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
                        (t.id).to_string().red().strikethrough(),
                        (t.task).to_string().red().strikethrough()
                    );
                    println!("[{}] {}", "X".bright_red(), formatted.dimmed());
                }
            }
        } else {
            eprintln!("{}", "Empty List!".bright_red().bold());
            std::process::exit(1);
        }
        Ok(())
    }

    // remove a existing todo
    pub fn delete(&self, index: &u32) -> Result<()> {
        if self.inc.contains(index) || self.com.contains(index) {
            self.conn.execute(
                "DELETE FROM todo WHERE id = ?1;",
                [index.to_string()],
            )?;
        } else {
            eprintln!("{}", "Please recheck the index!".bright_red());
            std::process::exit(1);
        }
        Ok(())
    }

    // To delete all todo from the database
    pub fn drop(&self) -> Result<()> {
        self.conn.execute("DROP TABLE todo;", ())?;
        Ok(())
    }

    // Modify an existing todo
    pub fn modify(&self, index: &u32, new: &str) -> Result<()> {
        if self.inc.contains(index) || self.com.contains(index) {
            self.conn.execute(
                "UPDATE todo SET task = ?1 WHERE id = ?2;",
                &[new, &index.to_string()],
            )?;
        } else {
            eprintln!("{}", "Please recheck the Index!".bright_red());
            std::process::exit(1);
        }
        Ok(())
    }

    // Ticks off a todo
    pub fn done(&self, index: &u32) -> Result<()> {
        if self.inc.contains(index) {
            self.conn.execute(
                "UPDATE todo SET done = 1 WHERE id = ?1;",
                [index.to_string()],
            )?;
        } else {
            eprintln!("{}", "Please recheck the Index!".bright_red());
            std::process::exit(1);
        }
        Ok(())
    }

    // show incomplete task
    pub fn incomplete(&self) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, task, done FROM todo WHERE done = 0;")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo { id: row.get(0)?, task: row.get(1)?, done: row.get(2)? })
        })?;
        println!("{}", "TO BE DONE".blue().underline());
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
        let mut stmt = self
            .conn
            .prepare("SELECT id, task, done FROM todo WHERE done = 1;")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo { id: row.get(0)?, task: row.get(1)?, done: row.get(2)? })
        })?;
        println!("{}", "COMPLETED".blue().underline());
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
