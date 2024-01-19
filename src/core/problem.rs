pub mod test;

use crate::core::program::Program;
use test::Test;

pub struct Problem {
    pub name: String,
    pub test: Test,
    pub checker: Option<Program>,
}
