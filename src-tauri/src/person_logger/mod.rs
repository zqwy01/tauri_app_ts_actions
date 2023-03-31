/// basically a person type. Note: the From<T> trait is implemented
/// with a tuple of (Username(String), Age(i32), Timestamp(String), Comment(String)), IN THIS ORDER!
use std::fs::OpenOptions;
use std::io::{Read, Write};

///```markdown 
///# A person struct
///Should only be used within this file, made private for a reason ✨ 
///```
struct Person {
    /// a field for a name
    name: String,
    /// a field for an age
    age: i32,
    /// an automatically generated field (by the frontend) for the timestamp
    timestamp: String,
    /// a field for the comment
    comment: String
}
pub struct PersonLogger {
    person: Person,
    target_file: String,
}
impl From<(String, i32, String, String)> for Person{
    fn from(value: (String, i32, String, String)) -> Self {
        Self {
            name: value.0,
            age: value.1,
            timestamp: value.2,
            comment: value.3,
        }
    }
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n --------------Person Info----------------\n
               \t - Username: {};\n
               \t - Age: {};\n
               \t - timestamp: {};\n
               \t - comment: {};\n
               -----------------------------------------\n",
               self.name, self.age, self.timestamp, self.comment)
    }
}
impl std::fmt::Display for PersonLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n
               Target file: {}\n",
               self.person,
               self.target_file)
    }
}
impl PersonLogger {
     /// takes a (Username(String), Age(i32), Timestamp(String), Comment(String)) tuple and a target file path.
     /// returns a PersonLogger instance
     /// Oh and btw, this struct is actually printable, i implemented
     /// std::fmt::Display on it! les goooo
     pub fn new(person_tuple: (String, i32, String, String), target_file: String) -> Self {
        Self {
            person: Person::from(person_tuple),
            target_file,
        }
     }
     /// ```markdown
     ///# PersonLogger::flush()
     /// Writes the PersonLogger data to a `self.target_file` (sadly uses cloning cuz i have no
     /// idea what to do otherwise)
     /// ```
     pub fn flush(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.target_file.clone())?;

        write!(file, "[{}]-Username:{}-Age:{}-Comment:{}\n", 
            self.person.timestamp,
            self.person.name,
            self.person.age,
            self.person.comment)
    }
}
