
use super::base::*;
pub enum Alpha {
    Ignore1(char),
    Ignores(&'static str),
    Jump,
    Alter1(char),
    Alters(&'static str),
    Flush,
    Reset,
    Push(char),
    Pop,
    Clear,
    Go(bool),   // true: front, false: back

}

pub trait Triggers {
    fn encountered_the_end(dat: Found) -> Alpha;
    fn per_unit(ch: char) -> Alpha;

}
