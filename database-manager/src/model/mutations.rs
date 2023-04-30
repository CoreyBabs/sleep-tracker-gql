use crate::DBManager;
use super::gql_types::*;

use async_graphql::{Context, Object};

pub struct MutationRoot;

// TODO: Mutations: add tag, add tag to sleep, add comment to sleep
// Update sleep (should this be seperate or same as adding tag/comment to sleep?), tag, comment
// Delete sleep, tag, comment
// Figure out input types

#[Object]
impl MutationRoot {
    async fn add_sleep(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Sleep input containing a night's data")] input: SleepInput)
        -> Option<Sleep> {
            let dbm = ctx.data_unchecked::<DBManager>();
            let sleep_id = dbm.insert_sleep(input.night.as_str(), input.amount, input.quality).await;

            if let Some(tags) = input.tags {
                dbm.add_tag_to_sleep(sleep_id, tags).await;
            }

            if let Some(comments) = input.comments {
                for comment in comments {
                    dbm.insert_comment(sleep_id, comment.as_str()).await;
                }
            }

            let sleep = dbm.get_sleep(sleep_id, false).await;
            match sleep {
                Some(s) => Some(Sleep::from_db(&s)),
                None => None,
            } 
        }
}