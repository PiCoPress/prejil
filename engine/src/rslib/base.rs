//! # Prejil
//! It takes `str`,
//! and it stores the data as `Chars`,
//! also results `String`.
//!

impl Base {
    /// It creates new instance which will be used to do something.
    ///
    /// # Example
    /// ```
    /// use prejil::rslib::base::*;
    ///
    /// let obj = Base::new("Any Strings");
    /// ```
    pub fn new(dat: &'static str) -> Base {
        Base {
            data: dat,
            cursor: 0,
        }
    }

    /// Seek towards front char `c` given and returns a chunk that it surrounds current and `c`, excluding `c`.
    ///
    /// # Examples
    /// ```
    /// # use prejil::rslib::base::*;
    ///
    /// let mut obj = Base::new("Any Strings");
    /// let result: String = if let Some(o) = obj.seek_char(' ') { o.data } else { "Error".to_string() };
    /// assert_eq!(result, "Any");
    ///
    /// let result: String = if let Some(o) = obj.seek_char('g') { o.data } else { "Error".to_string() };
    /// assert_eq!(result, " Strin");
    /// ```
    pub fn seek_char(&mut self, c: char) -> Option<Found> {
        let mut findings = String::new();
        for to_find in self.data[self.cursor as usize..].chars() {
            if c == to_find {
                return Some(Found {data: findings });
            }
            findings.push(to_find);
            self.cursor += 1;
        }
        None
    }

    /// Seek towards back char `c` given and returns a reversed chunk that it is surrounded current and `c`, excluding `c`.
    ///
    /// # Examples
    /// ```
    /// # use prejil::rslib::base::*;
    ///
    /// let mut obj = Base::new("Any Strings");
    /// let result: String = if let Some(o) = obj.seek_char(' ') { o.data } else { "Error".to_string() };
    /// assert_eq!(result, "Any");
    ///
    /// let result: String = if let Some(o) = obj.seek_char_back('n') { o.data } else { "Error".to_string() };
    /// assert_eq!(result, "y");
    /// ```
    pub fn seek_char_back(&mut self, c: char) -> Option<Found> {
        let mut findings: String = String::new();
        for to_find in self.data[..self.cursor as usize].chars().rev() {
            if c == to_find {
                return Some(Found {data: findings });
            }
            findings.push(to_find);
            self.cursor -= 1;
        }
        None
    }

    /// Returns current ASCII.
    pub fn get_current_byte(&self) -> u8 {
        self.data.as_bytes()[(self.cursor - 1) as usize]
    }

    /// Skip `count` numbers of data, also `count` can be negative.
    /// If refused, it returns false, else true.
    pub fn skip_chars(&mut self, count: i32) -> bool {
        if count < 0 {
            if count + self.cursor < 0 { return false }
        } else  if count + self.cursor > (self.data.len() - 1) as i32 { return false }

        self.cursor += count;
        true
    }
}
pub struct Base {
    data: &'static str,
    cursor: i32,
}
pub struct Found {
    pub data: String,
}