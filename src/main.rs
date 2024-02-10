pub mod execute;

fn main() {
    println!("Execute says {:?}", execute::hello());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hi() {
        assert_eq!(execute::hello(), "Kia ora from execute");
    }
}
