/// basically a person type. Note: the From<T> trait is implemented
/// with a tuple of (Username(String), Age(i32), Timestamp(String), Comment(String)), IN THIS ORDER!
use std::fs::OpenOptions;
use std::io::{Read, Write};

use serde::{Serialize, Deserialize};

///```markdown 
///# A person struct
///Should only be used within this file, made private for a reason ✨ 
///```
#[derive(Serialize, Deserialize)]
struct Person {
    /// a field for a username
    username: String,
    /// a field for an age
    age: i32,
    /// an automatically generated field (by the frontend) for the timestamp
    timestamp: String,
    /// a field for the comment
    comment: String
}
#[derive(Serialize, Deserialize)]
pub struct PersonLogger {
    persons: Option<Vec<Person>>,
    target_file: String,
}
impl From<(String, i32, String, String)> for Person{
    fn from(value: (String, i32, String, String)) -> Self {
        Self {
            username: value.0,
            age: value.1,
            timestamp: value.2,
            comment: value.3,
        }
    }
}
impl From<&(String, i32, String, String)> for Person{
    fn from(value: &(String, i32, String, String)) -> Self {
        Self {
            username: value.0.clone(),
            age: value.1,
            timestamp: value.2.clone(),
            comment: value.3.clone(),
        }
    }
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------------Person Info----------------\n
               \t - Username: {};\n
               \t - Age: {};\n
               \t - timestamp: {};\n
               \t - comment: {};\n-----------------------------------------\n",
               self.username, self.age, self.timestamp, self.comment)
    }
}
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------------Person Info----------------\n
               \t - Username: {};\n
               \t - Age: {};\n
               \t - timestamp: {};\n
               \t - comment: {};\n-----------------------------------------\n",
               self.username, self.age, self.timestamp, self.comment)
    }
}
impl std::default::Default for Person {
    fn default() -> Self {
        Self {
            username: "John Doe".to_owned(),
            age: 34,
            timestamp: "1.01.1970, 00:00:00".to_owned(),
            comment: "An error must have occured somewhere right?".to_owned(),
        }
    }    
}
impl std::fmt::Display for PersonLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}\n
               Target file: {}\n",
               self.persons.as_ref().unwrap(),
               self.target_file)
    }
}
impl PersonLogger {
     /// takes a (Username(String), Age(i32), Timestamp(String), Comment(String)) tuple and a target file path.
     /// returns a PersonLogger instance
     /// Oh and btw, this struct is actually printable, i implemented
     /// std::fmt::Display on it! les goooo
     pub fn from_tuple(persons_vec_json: Vec<String>, target_file: String) -> Self {
        let persons: Vec<Person> = persons_vec_json.iter().map(
            |p| -> Person {
                match serde_json::from_str(p) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Error during parsing: {e}");
                        Person::default()
                    }
                }
            }
        ).collect();
        Self {
            persons: Some(persons),
            target_file,
        }
     }
     ///```markdown
     ///PersonLogger::new_empty()
     ///
     /// Creates an empty PersonLogger instance, used for state management basically
     /// ```
     pub fn new_empty(target_file: String) -> Self {
        Self {
            persons: None,
            target_file,
        }
     }
     /// ```markdown
     /// PersonLogger::append()
     ///
     /// Appends a given Vec<String, i32, String, String> to PersonLogger
     /// if Option<Vec<Person>> is None, replaces it with person_tuple to persons_given_array
     /// if Option<Vec<Person>> is Some(n), append given to that
     /// ```
     pub fn append(&mut self, persons_vec_json: Vec<String>) {
        let mut persons_given_array: Vec<Person> = persons_vec_json.iter().map(
            |p| -> Person {
                match serde_json::from_str(p) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Error during parsing: {e}");
                        Person::default()
                    }
                }
            }
        ).collect();
        if let Some(persons) = &mut self.persons {
            persons.append(&mut persons_given_array);
        } else {
            self.persons = Some(persons_given_array);
        }
     }
     /// ```markdown
     ///# PersonLogger::flush()
     /// Writes the PersonLogger data to a `self.target_file` (sadly uses cloning cuz i have no
     /// idea what to do otherwise)
     /// ```
     pub fn flush(&mut self) -> std::io::Result<()> {
        // this if let Some() there is to avoid looping over nothin lol
        // if removed
        // then wont work lol
        // AFTER FLUSH THE PersonLogger.persons IS EMPTIED
        if let Some(persons) = &self.persons { 
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(self.target_file.clone())?;
            for person in persons { 
                match write!
                    (
                    file, "[{}]-Username:{}-Age:{}-Comment:{}\n", 
                    person.timestamp,
                    person.username,
                    person.age,
                    person.comment) {
                    Ok(()) => {},
                    Err(e) => println!("{:#?}", e),
                };
            }
            self.persons = None;
            Ok(())} else {
                Err(std::io::ErrorKind::InvalidData.into())
            }
    }
}
