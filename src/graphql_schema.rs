use juniper::{EmptyMutation, RootNode};

struct Member {
    id: i32,
    name: String
}

#[juniper::object(description = "A member of a team")]
impl Member {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn members() -> Vec<Member> {
        vec![
            Member {
                id: 1,
                name: "Charlie".to_owned(),
            },
            Member {
                id: 2,
                name: "Frank".to_owned(),
            }
        ]
    } 
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}

pub fn test() -> Option<String> {
    println!("Runs");
    Some(String::from("Hello"))
}
