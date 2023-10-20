mod tests {
    use std::sync::{Arc, RwLock};
    #[cfg_attr(test, mockall_wrap::wrap, mockall::automock)]
    pub trait TestTrait: Send + Sync {
        fn test_method(&self, arg: u32) -> u32;
        fn test_method_2(&mut self, arg: u32) -> u32;
        fn clone_box(&self) -> Box<dyn TestTrait>;
    }
    #[test]
    fn test_basic() {}
}
