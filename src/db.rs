use anyhow::Result;
use rusqlite::Connection;

const DB_NAME: &str = "motionsensor.db";
const TABLE_NAME: &str = "logs";

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn init(db_path: &str) -> Result<DB> {
        let mut path = db_path.to_string();

        if !db_path.ends_with("/") {
            path = format!("{db_path}/");
        }

        let conn = Connection::open(format!("{path}{DB_NAME}"))?;

        conn.execute(
            format!(
                "create table if not exists {TABLE_NAME} (
					id INTEGER primary key,
					timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                    images_path TEXT,
					notes TEXT
				)"
            )
            .as_str(),
            (),
        )?;

        Ok(DB { conn })
    }

    pub fn log(&mut self, images_path: &str) -> Result<()> {
        let _res = &self.conn.execute(
            format!("insert into {TABLE_NAME}(images_path) values(?1)").as_str(),
            &[&images_path],
        )?;

        Ok(())
    }
}
