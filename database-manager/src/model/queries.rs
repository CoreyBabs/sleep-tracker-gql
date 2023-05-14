use crate::DBManager;
use super::gql_types::*;

use async_graphql::{Context, Object};

/// Contains the query definitions for the graphql api.
pub struct QueryRoot;

#[Object]
impl QueryRoot {

    /// Get all sleeps
    async fn all_sleeps<'a>(&self, ctx: &Context<'a>) -> Option<Vec<Sleep>> {
        let dbm = ctx.data_unchecked::<DBManager>();
        let sleeps = dbm.get_all_sleeps().await;
        sleeps.map(|v| v.iter().map(Sleep::from_db).collect::<Vec<Sleep>>())
    }

    /// Get the sleep with the given id
    async fn sleep<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "id of the sleep")] id: i64) 
        -> Option<Sleep> {
        let dbm = ctx.data_unchecked::<DBManager>();
        Sleep::from_sleep_id(dbm, id).await
    }

    /// Get Sleeps in a given month
    async fn sleeps_by_month<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Month and year to get sleeps from.")] month: SleepsByMonthInput)
        -> Option<Vec<Sleep>> {
            let dbm = ctx.data_unchecked::<DBManager>();
            let sleeps = dbm.get_sleeps_by_month(month.month, month.year).await;
            sleeps.map(|v| v.iter().map(Sleep::from_db).collect::<Vec<Sleep>>())
        }

    /// Get sleeps in a given date range. Dates are inclusive
    async fn sleeps_in_range<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Inclusive start date of date range.")] start_date: SleepsInRangeInput,
        #[graphql(desc = "Inclusive end date of date range.")] end_date: SleepsInRangeInput)
        -> Option<Vec<Sleep>> {
            let dbm = ctx.data_unchecked::<DBManager>();
            let sleeps = dbm.get_all_sleeps().await;
            let sleeps = sleeps.map(|v| v.iter().map(Sleep::from_db).collect::<Vec<Sleep>>());
            Sleep::filter_sleeps_by_date(sleeps, &start_date, &end_date)
        }

    /// Get the tag with the given id
    async fn tag<'a>(
        &self,
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
        tags.map(|v| v.iter().map(|t| Tag { id: t.id, name: t.name.clone(), color: t.color})
            .collect::<Vec<Tag>>())
    }
}