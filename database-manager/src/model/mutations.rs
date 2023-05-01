use crate::DBManager;
use super::gql_types::*;

use async_graphql::{Context, Object};

pub struct MutationRoot;

// TODO: Mutations: Update sleep (should this be seperate or same as adding tag/comment to sleep?), tag, comment
// Delete sleep, tag, comment
// abstract returning of sleep

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
                dbm.add_tags_to_sleep(sleep_id, tags).await;
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

        async fn add_tag(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Tag input containing a tag's data")] input: TagInput)
            -> Option<Tag> {
                let dbm = ctx.data_unchecked::<DBManager>();
                let tag_id = dbm.insert_tag(input.name.as_str(), input.color).await;
    
                let tag = dbm.get_tag(tag_id).await;
                match tag {
                    Some(t) => Some(Tag { id: t.id, name: t.name.clone(), color: t.color }),
                    None => None,
                } 
            }

        async fn add_tags_to_sleep(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Sleep id to add tag to.")] sleep_id: i64,
            #[graphql(desc = "Tag ids to add to sleep")] tag_ids: Vec<i64>)
            -> Option<Sleep> {
                let dbm = ctx.data_unchecked::<DBManager>();
                dbm.add_tags_to_sleep(sleep_id, tag_ids).await;

                let sleep = dbm.get_sleep(sleep_id, false).await;
                match sleep {
                    Some(s) => Some(Sleep::from_db(&s)),
                    None => None,
                }
            }
        
        async fn add_comment_to_sleep(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Sleep id to add tag to.")] sleep_id: i64,
            #[graphql(desc = "Comment to add to sleep")] comment: String)
            -> Option<Sleep> {
                let dbm = ctx.data_unchecked::<DBManager>();
                dbm.insert_comment(sleep_id, comment.as_str()).await;

                let sleep = dbm.get_sleep(sleep_id, false).await;
                match sleep {
                    Some(s) => Some(Sleep::from_db(&s)),
                    None => None,
                }
            }
}