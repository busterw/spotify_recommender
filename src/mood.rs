use std::{env, path::PathBuf};
use rusqlite::{params, Connection, Result, Error};

pub struct MoodParams{
    pub valence: f32
}

pub fn mood_to_params(mood: &str) -> Result<Option<MoodParams>> {
    let mut db_path: PathBuf = env::current_dir().unwrap();  // Propagate any error from `current_dir`
    db_path.push("Moods.db3");                       // Append the filename

    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT Valence FROM Moods WHERE Name = ?1")?;

    match stmt.query_row(params![mood], |row| {
        Ok(MoodParams {
            valence: row.get(0)?,
        })
    }) {
        Ok(mood_param) => Ok(Some(mood_param)),           // Return the found result
        Err(Error::QueryReturnedNoRows) => Ok(None),      // Return None if no row is found
        Err(e) => Err(e),                                 // Propagate any other errors
    }
}
