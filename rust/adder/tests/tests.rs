#[cfg(test)]
mod tests {
    use adder::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn failing_test() {
        panic!("make this test fail");
    }
}
