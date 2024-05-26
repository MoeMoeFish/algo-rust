use std::collections::LinkedList;
use crate::utils::TestStruct;


#[cfg(test)]
mod test {
    use std::collections::LinkedList;
    use crate::utils::TestStruct;

    #[test]
    fn test_linked_list() {
        let ll: LinkedList<TestStruct> = LinkedList::new();
    }

    #[test]
    fn test_dyn_01() {
        use std::fmt::Debug;

        // 使用 Box<T> where T: Debug
        fn print_debug_known_type<T>(item: Box<T>) where T: Debug {
            println!("{:?}", item);
        }

        // 使用 Box<dyn Debug>
        fn print_debug_any_type(item: Box<dyn Debug>) {
            println!("{:?}", item);
        }

        let x = Box::new(42); // Box<i32>
        print_debug_known_type(x);

        let y = Box::new("hello"); // Box<&str>，它实现了 Debug
        print_debug_any_type(y);
    }
}