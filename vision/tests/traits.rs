

struct TestStr {
    id: i32,
    name: String
}


trait GetData {
    fn get_id(&self) -> i32;
}

impl GetData for TestStr {

    fn get_id(&self) -> i32 {
       self.id 
    }
}



#[test]
fn trait_test() {
    let test = TestStr {id: 1, name: "test".to_string() };
    assert_eq!(1, test.get_id())
}

