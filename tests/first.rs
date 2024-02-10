use dilbert::execute;

#[test]
fn test_hello() {
    assert_eq!("Kia ora from execute", execute::hello())
}
