pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn trigger_panic() {
    panic!("Panic!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn adds_two() {
        assert_eq!(crate::add_two(2), 4);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        crate::trigger_panic(); 
    }
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
