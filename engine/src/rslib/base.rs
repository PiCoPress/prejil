struct Base {
    cursor: u32,
    flag: i8,
    data: str,
    value: u16,
    length: u32,
}
impl Base {
    fn seek_char(&mut self, c: &char) {
        for to_find in c[self.cursor..] {
            if c == to_find {
                self.flag = 1;
                return;
            }
            self.cursor += 1;
        }
        std::process::exit(2);
    }
    fn seek_back(&mut self, c: &char) {
        let c = vec!(c).iter().rev();
        for to_find in c[..self.cursor] {
            if c == to_find {
                self.flag = 1;
                return;
            }
            self.cursor += 1;
        }
        std::process::exit(2);
    }
}