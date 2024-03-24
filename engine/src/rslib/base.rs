impl Base {
    pub fn init(dat: &'static str) -> Base {
        Base {
            cursor: 0,
            data: dat,
        }
    }
    pub fn seek_char(&mut self, c: char) -> Option<Found> {
        let mut bind: Vec<char> = Vec::new();
        let c: u8 = c as u8;
        let ii = self.cursor + 1;
        for to_find in self.data[ii..].as_bytes().iter() {
            if &c == to_find {
                return Some(Found {value: bind});
            }
            bind.push(char::from(*to_find));
            self.cursor += 1;
        }
        self.cursor = ii - 1;
        return None;
    }
    pub fn seek_char_back(&mut self, c: char) -> Option<Found> {
        let mut ii = self.cursor;
        let c: u8 = c as u8;
        let mut bind: Vec<char> = Vec::new();
        let self_copy = self.data.as_bytes();
        while ii > 0 {
            if self.data.as_bytes()[ii - 1] == c {
                self.cursor = ii;
                return Some(Found {value: bind});
            }
            bind.push(self_copy[ii - 1] as char);
            ii -= 1;
        }
        return None;
    }
}
pub struct Base {
    cursor: usize,
    data: &'static str,
}
pub struct Found {
    pub(crate) value: Vec<char>,
}
