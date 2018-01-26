//! AN SQL string generator module
//! 
//! Each database backend generates different SQL syntax and abstracts
//! the differences in implementations away from the migration
//! implementation.
//! 
//! The central traits are contained in the root of this module to provide
//! interoperability between different database backends

pub mod postgres;


/// A module which generates SQL syntax focused around generating 
/// basic SQL database statements
pub trait DatabaseGenerator {

    /// Create a new table with a name
    fn create_table(name: &str) -> String;

    /// Create a new table with a name, only if it doesn't exist
    fn create_table_if_not_exists(name: &str) -> String;

    /// Drop a table with a name 
    fn drop_table(name: &str) -> String;

    /// Drop a table with a name, only if it exists
    fn drop_table_if_exists(name: &str) -> String;

    /// Rename a table from <old> to <new>
    fn rename_table(old: &str, new: &str) -> String;

    /// Modify a table in some other way
    fn modify_table(name: &str) -> String;
}


/// A module which generates SQL syntax foused around generating
/// table manipulation statements
pub trait TableGenerator {

    /// Drop an existing column from the table
    fn drop_column(name: &str) -> String;

    /// Rename an existing column
    fn rename_column(old: &str, new: &str) -> String;

    /// Add an auto-incrementing primary key
    fn increments() -> String;
    
    /// Add an integer column
    fn integer(name: &str) -> String;
    
    /// Add a text column
    fn text(name: &str) -> String;
    
    /// Add a string column
    fn string(name: &str) -> String;
    
    /// Add a timestamp column
    fn timestamp(name: &str) -> String;
}









// fn big_integer(-> String;
// fn float(&mut self) -> String;
// fn decimal(&mut self) -> String;
// fn boolean(&mut self) -> String;
// fn date(&mut self) -> String;
// fn date_time(&mut self) -> String;
// fn time(&mut self) -> String;
// fn timestamps(&mut self) -> String;
// fn drop_timestamps(&mut self) -> String;
// fn binary(&mut self) -> String;
// fn enumerable(&mut self) -> String;
// fn json(&mut self) -> String;
// fn jsonb(&mut self) -> String;
// fn uuid(&mut self) -> String;
// fn comment(&mut self) -> String;
// fn engine(&mut self) -> String;
// fn charset(&mut self) -> String;
// fn collate(&mut self) -> String;
// fn inherits(&mut self) -> String;
// fn specific_type(&mut self) -> String;
// fn index(&mut self) -> String;
// fn drop_index(&mut self) -> String;
// fn unique(&mut self) -> String;
// fn foreign(&mut self) -> String;
// fn drop_foreign(&mut self) -> String;
// fn drop_unique(&mut self) -> String;
// fn drop_primary(&mut self) -> String;