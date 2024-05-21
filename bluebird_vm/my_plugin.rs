pub struct Person {
    name: String,
    age: u8
}

impl Person {
    pub fn say_hello(&self) {
        println!("Hello MR.{}", self.name);
    }
}