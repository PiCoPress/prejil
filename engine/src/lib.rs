mod rslib;

#[cfg(test)]
mod tests {
    use super::*;
    use rslib::base::*;
    #[test]
    fn it_works() {
        let mut t = Base::init("araabaaq");
        let res = t.seek_char('b');
        assert_eq!(match res {
            Some(ct) => ct.value,
            None => vec!('s')
        }, vec!['r', 'a', 'a']);
    }

    #[test]
    fn copy_paste_char_vec() {
        let input = b"dsd";
        print!("[");
        for i in input[..input.len() - 2].iter() {
            print!("'{}', ", *i as char);
        }
        println!("'{}']", input[input.len() - 1] as char);
        assert_eq!(2, 1);
    }
}
