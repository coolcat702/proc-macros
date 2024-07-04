use builder::Builder;

#[derive(Debug, Builder)]
struct Person {
    #[builder(default = "1")]
    age: u32,
    job: Option<String>,
    name: String,
}

fn main() {
    let alice = Person::builder()
        .age(30)
        .job(Some("Engineer".to_string()))
        .name("Alice".to_string())
        .build()
        .unwrap();
    println!("{:?}", alice);
}