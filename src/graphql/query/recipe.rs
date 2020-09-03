use juniper::object;
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
}

#[object(Context=crate::graphql::context::Context)]
impl Recipe {
    fn id(&self) -> i32 {
        self.id.try_into().unwrap()
    }

    fn name(&self) -> &str {
        &self.name
    }
}
