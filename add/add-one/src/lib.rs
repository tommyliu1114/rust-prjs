#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //assert_eq!(2 + 2, 4);
        assert_eq!(3,super::add_one(2));
    }
}

pub fn add_one(x: i32) -> i32{
    x + 1
}