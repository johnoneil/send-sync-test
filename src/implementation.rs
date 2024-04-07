use crate::api::MyAPI;

pub struct MyImpl {
    value: i32,
}

impl MyImpl {
    pub fn new() -> MyImpl {
        MyImpl { value: 1234 }
    }
}

impl MyAPI for MyImpl {
    fn do_something(&mut self) -> i32 {
        self.value += 1;
        println!("my value: {}", self.value);
        self.value
    }
}

// A custom Drop impl so we can see the drop
impl Drop for MyImpl {
    fn drop(&mut self) {
        println!("Custom drop fired.");
    }
}
