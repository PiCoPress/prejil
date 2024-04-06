use crate::rslib::base::Found;
use crate::rslib::groups::BlockGroups;

type Token = &'static Found;
pub enum Beta {
    Expect(Token),
    Expects(BlockGroups),
    Ignore(Token),
    Prev,
    Next,
    NoExpect(Token),
    NoExpects(BlockGroups),
}

pub trait Triggers {
    fn per_unit(t: Token) -> Beta;
}