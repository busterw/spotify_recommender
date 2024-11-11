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

// pub fn load_vader_data() -> Result<()> {
//     // Open the VADER sentiment file
//     let mut file_path: PathBuf = env::current_dir().unwrap();  // Propagate any error from `current_dir`
//     file_path.push("vader_lexicon.txt");

//    let path_str = file_path.display().to_string();

//    println!("{}", path_str);

//     let file = File::open(file_path).unwrap();  // Replace with your actual file path
    
//     let reader = BufReader::new(file);

//     let mut db_path: PathBuf = env::current_dir().unwrap();  // Propagate any error from `current_dir`
//     db_path.push("Moods.db3");                       // Append the filename

//     let conn = Connection::open(db_path)?;

//     // Begin a transaction for efficiency
//     conn.execute("BEGIN TRANSACTION;", [])?;

//     for line in reader.lines() {
//         let line = line.unwrap();  // Handle errors reading lines

//         // Skip lines that start with a comment (assuming comments are prefixed with `$`)
//         if line.starts_with('$') {
//             continue;
//         }

//         // Split the line into words and values
//         let parts: Vec<&str> = line.split_whitespace().collect();
//         if parts.len() < 2 {
//             continue;  // Skip invalid lines
//         }

//         let word = parts[0].to_string();  // First column (word)
//         let valence: f32 = parts[1].parse().unwrap_or(0.0);  // Second column (valence), defaulting to 0.0 on parse error

//         // Normalize the valence to be between 0 and 1
//         let normalized_valence = (valence + 4.0) / 8.0;

//         // Insert the data into the database
//         conn.execute(
//             "INSERT INTO Moods (Name, Valence) VALUES (?1, ?2)",
//             params![word, normalized_valence],
//         )?;
//     }

//     // Commit the transaction
//     conn.execute("COMMIT;", [])?;

//     Ok(())
// }
