#[cfg(test)]

mod tests {
    use crate::function::{add, sub, printf};

    #[test]
    fn test_add(){
        assert_eq!(add(2,3), 5);
    }
    #[test]
    fn test_sub()
    {
        assert_eq!(sub(5,3), 2);
        assert_eq!(sub(5.0, 3.0), 2.0);
    }
}