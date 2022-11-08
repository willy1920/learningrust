use async_graphql::Object;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn test(&self) -> &str {
        "hello world"
    }

    async fn humans(&self) -> &str {
        "humans"
    }
}
