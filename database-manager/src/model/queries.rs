use crate::DBManager;
use super::gql_types::*;

use async_graphql::{Context, Object};

pub struct QueryRoot;

#[Object]
impl QueryRoot {

    /// Get all sleeps
    async fn all_sleeps<'a>(&self, ctx: &Context<'a>) -> Option<Vec<Sleep>> {
        let dbm = ctx.data_unchecked::<DBManager>();
        let sleeps = dbm.get_all_sleeps().await;
        match sleeps {
            Some(v) => {
                Some(v.iter().map(Sleep::from_db).collect::<Vec<Sleep>>())
            },
            None => None
        }
    }

    /// Get the sleep with the given id
    async fn sleep<'a>(&self,
        ctx: &Context<'a>,
        #[graphql(desc = "id of the sleep")] id: i64) 
        -> Option<Sleep> {
        let dbm = ctx.data_unchecked::<DBManager>();
        Sleep::from_sleep_id(dbm, id).await
    }

    /// Get the tag with the given id
    async fn tag<'a>(&self,
        ctx: &Context<'a>,
        #[graphql(desc = "id of the tag")] id: i64) 
        -> Option<Tag> {
        let dbm = ctx.data_unchecked::<DBManager>();
        let tag = dbm.get_tag(id).await;
        match tag {
            Some(t) => Some(Tag { id: t.id, name: t.name.clone(), color: t.color}),
            None => None,
        }
    }

    /// Get all tags
    async fn all_tags<'a>(&self, ctx: &Context<'a>) -> Option<Vec<Tag>> {
        let dbm = ctx.data_unchecked::<DBManager>();
        let tags = dbm.get_all_tags().await;
        match tags {
            Some(v) => {
                Some(v.iter().map(|t| Tag { id: t.id, name: t.name.clone(), color: t.color})
                    .collect::<Vec<Tag>>())
            },
            None => None
        }
    }
}