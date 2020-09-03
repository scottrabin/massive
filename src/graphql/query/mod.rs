mod recipe;

pub struct Query;

#[juniper::object(Context = super::context::Context)]
impl Query {
    async fn recipes() -> Vec<recipe::Recipe> {
        vec![recipe::Recipe {
            id: 1,
            name: "omelette".into(),
        }]
    }
}
