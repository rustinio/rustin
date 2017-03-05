/// A user of a chat service.
#[derive(Clone, Debug)]
pub struct User {
    id: String,
    name: Option<String>,
}

impl User {
    pub fn new<I, N>(id: I, name: Option<N>) -> Self where I: Into<String>, N: Into<String> {
        User {
            id: id.into(),
            name: name.map(|n| n.into()),
        }
    }
}
