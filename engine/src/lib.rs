pub mod rslib;

#[cfg(test)]
mod tests {
    use super::*;
    use rslib::base::*;

    #[test]
    fn it_works() {
        let mut t = Base::new("araabaaq");
        let res = t.seek_char('b');
        assert_eq!(match res {
            Some(ct) => ct.data,
            None => String::new()
        }, "araa".to_string());
    }
}
