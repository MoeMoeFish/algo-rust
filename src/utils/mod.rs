#[allow(unused)]
pub (crate) struct TestStruct {
    id: i32
}

impl TestStruct {
    #[allow(unused)]
    fn new(id: i32) -> Self {
        Self {
            id
        }
    }
}