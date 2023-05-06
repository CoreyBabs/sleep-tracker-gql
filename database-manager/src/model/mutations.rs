use crate::DBManager;
use super::gql_types::*;

use async_graphql::{Context, Object};

pub struct MutationRoot;

// TODO: Mutations:
// Remove tag from sleep

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

            Sleep::from_sleep_id(dbm, sleep_id).await 
        }

        async fn add_tag(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Tag input containing a tag's data")] input: TagInput)
            -> Option<Tag> {
                let dbm = ctx.data_unchecked::<DBManager>();
                let tag_id = dbm.insert_tag(input.name.as_str(), input.color).await;
    
                Tag::from_tag_id(dbm, tag_id).await 
            }

        async fn add_tags_to_sleep(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Sleep id to add tag to.")] sleep_id: i64,
            #[graphql(desc = "Tag ids to add to sleep")] tag_ids: Vec<i64>)
            -> Option<Sleep> {
                let dbm = ctx.data_unchecked::<DBManager>();
                dbm.add_tags_to_sleep(sleep_id, tag_ids).await;

                Sleep::from_sleep_id(dbm, sleep_id).await 
            }
        
        async fn add_comment_to_sleep(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Sleep id to add tag to.")] sleep_id: i64,
            #[graphql(desc = "Comment to add to sleep")] comment: String)
            -> Option<Sleep> {
                let dbm = ctx.data_unchecked::<DBManager>();
                dbm.insert_comment(sleep_id, comment.as_str()).await;

                Sleep::from_sleep_id(dbm, sleep_id).await 
            }

        async fn delete_sleep(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Sleep id to delete.")] sleep_id: i64)
            -> bool {
                let dbm = ctx.data_unchecked::<DBManager>();
                dbm.delete_sleep(sleep_id).await
        }

        async fn delete_tag(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "tag id to delete.")] tag_id: i64)
            -> bool {
                let dbm = ctx.data_unchecked::<DBManager>();
                dbm.delete_tag(tag_id).await
        }

        async fn delete_comment(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "comment id to delete.")] comment_id: i64)
            -> bool {
                let dbm = ctx.data_unchecked::<DBManager>();
                dbm.delete_comment(comment_id).await
        }

        async fn update_sleep(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Sleep to edit. Non none fields will be updated.")] sleep_input: UpdateSleepInput)
            -> Option<Sleep> {
                let dbm = ctx.data_unchecked::<DBManager>();
                
                let sleep_id = sleep_input.sleep_id;
                let optional_quality = sleep_input.quality;
                let optional_amount = sleep_input.amount;

                let quality_updated = match optional_quality {
                    Some(quality) => dbm.update_sleep_quality(sleep_id, quality).await,
                    None => false
                };

                let amount_updated = match optional_amount {
                    Some(amount) => dbm.update_sleep_amount(sleep_id, amount).await,
                    None => false
                };

                if quality_updated || amount_updated {
                    Sleep::from_sleep_id(dbm, sleep_id).await
                }
                else {
                    None
                }
            }

        async fn update_tag(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Tag to edit. Non none fields will be updated.")] tag_input: UpdateTagInput)
            -> Option<Tag> {
                let dbm = ctx.data_unchecked::<DBManager>();
                
                let tag_id = tag_input.tag_id;
                let optional_name = tag_input.name;
                let optional_color = tag_input.color;

                let name_updated = match optional_name {
                    Some(name) => dbm.update_tag_name(tag_id, name.as_str()).await,
                    None => false
                };

                let color_updated = match optional_color {
                    Some(color) => dbm.update_tag_color(tag_id, color).await,
                    None => false
                };

                if name_updated || color_updated {
                    Tag::from_tag_id(dbm, tag_id).await
                }
                else {
                    None
                }
            }

        async fn update_comment(
            &self,
            ctx: &Context<'_>,
            #[graphql(desc = "Comment to edit.")] comment_input: UpdateCommentInput)
            -> Option<Comment> {
                let dbm = ctx.data_unchecked::<DBManager>();
                let comment_updated = dbm.update_comment(comment_input.comment_id, comment_input.comment.as_str()).await;
                if comment_updated {
                    Comment::from_comment_id(dbm, comment_input.comment_id).await
                }
                else {
                    None 
                }
            }
}