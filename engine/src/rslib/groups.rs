use crate::rslib::base::Found;

pub struct CharGroups {
    pub list: Vec<char>,
}

pub struct StrGroups {
    pub list: Vec<&'static str>,
}

pub struct BlockGroups {
    pub list: Vec<Found>,
}