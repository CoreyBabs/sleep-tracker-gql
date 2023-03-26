use async_graphql::{Context, Object};

use super::DBManager;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn error<'a>(
        &self,
        ctx: &Context<'a>
    ) -> String {
        let dbm = ctx.data_unchecked::<DBManager>();
        dbm.get_last_error().to_owned()
    }
}