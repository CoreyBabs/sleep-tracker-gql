use async_graphql::{Context, Object, SimpleObject, InputObject};
use crate::db_manager::DbmSleep;
use crate::DBManager;

/// Graphql representation of a sleep
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sleep {
    /// Primary key
    pub id: i64,

    /// Date of the night
    pub night: Night,

    /// Amount of sleep
    pub amount: f64,

    /// Quality of sleep, scale is flexible
    pub quality: i64,

    /// Tags assoicated to the sleep
    pub tags: Option<Vec<Tag>>,

    /// Comments associated to the sleep
    pub comments: Option<Vec<Comment>>
}

impl Sleep {
    pub fn from_db(db_sleep: &DbmSleep) -> Sleep {
        let tags = match &db_sleep.tags {
            Some(v) => {
                let tag = v.iter().map(|t| Tag { id: t.id, name: t.name.clone(), color: t.color }).collect::<Vec<Tag>>();
                Some(tag)
            },
            None => None,
        };

        Sleep {
            id: db_sleep.sleep.id,
            night: Night::from_string(db_sleep.sleep.night.clone()),
            amount: db_sleep.sleep.amount,
            quality: db_sleep.sleep.quality,
            tags,
            comments: None
        }
    }

    pub async fn from_sleep_id(dbm: &DBManager, sleep_id: i64) -> Option<Sleep> {
        let sleep = dbm.get_sleep(sleep_id, false).await;
        sleep.map(|s| Sleep::from_db(&s)) 
    }

    pub fn filter_sleeps_by_date(
        sleeps: Option<Vec<Sleep>>,
        start_date: &SleepsInRangeInput,
        end_date: &SleepsInRangeInput)
        -> Option<Vec<Sleep>> {
            sleeps.map(|v| v.into_iter()
                .filter(|s| s.night.in_date_range(start_date, end_date))
                .collect::<Vec<Sleep>>())
        }
}

#[Object]
impl Sleep {
    async fn id(&self) -> i64 {
        self.id
    }

    async fn night(&self) -> Night {
        self.night.clone()
    }

    async fn amount(&self) -> f64 {
        self.amount
    }

    async fn quality(&self) -> i64 {
        self.quality
    }

    async fn tags(&self, ctx: &Context<'_>) -> Option<Vec<Tag>> {
        let dbm = ctx.data_unchecked::<DBManager>();
        let tags = dbm.get_tags_by_sleep(self.id).await;
        let tags = match tags {
            Some(v) => {
                let tag = v.iter().map(|t| Tag { id: t.id, name: t.name.clone(), color: t.color }).collect::<Vec<Tag>>();
                Some(tag)
            },
            None => None,
        };
        tags
    }

    async fn comments(&self, ctx: &Context<'_>) -> Option<Vec<Comment>> {
        let dbm = ctx.data_unchecked::<DBManager>();
        let comments = dbm.get_comments_by_sleep(self.id).await;
        match comments {
            Some(v) => {
                let comment = v.iter().map(|c| 
                    Comment { id: c.id, sleep_id: c.sleep_id, comment: c.comment.clone() }).collect::<Vec<Comment>>();
                Some(comment)
            },
            None => None,
        }
    }
}

/// Graphql representation of a tag
#[derive(Debug, Clone, Default, PartialEq, SimpleObject)]
pub struct Tag {
    /// Primary key
    pub id: i64,

    /// Name of the tag
    pub name: String,

    /// Decimal representation of the rgb color value for the tag ex: Red (0xFF0000) is 16711680
    pub color: i64,
}

impl Tag {
    pub async fn from_tag_id(dbm: &DBManager, tag_id: i64) -> Option<Tag> {
        let tag = dbm.get_tag(tag_id).await;
        match tag {
            Some(t) => Some(Tag { id: t.id, name: t.name.clone(), color: t.color }),
            None => None,
        } 
    }
}

/// Graphql representation of a comment
#[derive(Debug, Clone, Default, PartialEq, SimpleObject)]
pub struct Comment {
    /// Primary key
    pub id: i64,

    /// id of the sleep the comment is associated to
    pub sleep_id: i64,

    /// text comment
    pub comment: String,
}

impl Comment {
    pub async fn from_comment_id(dbm: &DBManager, comment_id: i64) -> Option<Comment> {
        let comment = dbm.get_comment(comment_id).await;
        match comment {
            Some(c) => Some(Comment {id: c.id, sleep_id: c.sleep_id, comment: c.comment}),
            None => None
        }
    }
}

/// Graphql representation of the date
#[derive(Debug, Clone, Default, PartialEq, SimpleObject)]
pub struct Night {
    /// Day portion of the date
    pub day: u8,

    /// Month portion of the date
    pub month: u8,

    /// Year portion of the date
    pub year: u16,

    /// String representation of the date in yyyy-mm-dd format
    pub date: String,
}

impl Night {
    pub fn from_string(night: impl Into<String>) -> Night {
        let date = night.into();
        let night: Vec<&str> = date.split('-').collect();
        
        // TODO: Unwrapping here is not safe, so this should be handled better
        Night {
            day: night[2].parse::<u8>().unwrap(),
            month: night[1].parse::<u8>().unwrap(),
            year: night[0].parse::<u16>().unwrap(),
            date
        }
    }

    pub fn in_date_range(&self, start_date: &SleepsInRangeInput, end_date: &SleepsInRangeInput) -> bool {

        let in_year_range = self.year >= start_date.year && self.year <= end_date.year;
        let in_month_range = self.month >= start_date.month && self.month <= end_date.month;
        let in_day_range = match (start_date.day, end_date.day) {
            (Some(s), Some(e)) => self.day >= s && self.day <= e, 
            (_, _) => true
        };

        in_year_range && in_month_range && in_day_range
    }
}

/// Graphql representation for inputting a sleep to the database
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct SleepInput {
    /// String representation of the date in yyyy-mm-dd format
    pub night: String,

    /// Amount of sleep
    pub amount: f64,

    /// Quality of sleep
    pub quality: i64,

    /// Tags to associate to the sleep
    pub tags: Option<Vec<i64>>,

    /// Comments to add to the sleep
    pub comments: Option<Vec<String>>
}

/// Graphql representation for inputting a tag to the database
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct TagInput {
    /// Name of the tag
    pub name: String,

    /// Decimal representation of the rgb color value for the tag ex: Red (0xFF0000) is 16711680
    pub color: i64,
}

/// Graphql input for adding tags to a sleep
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct AddTagsToSleepInput {
    /// Id of the sleep to add the tags to
    pub sleep_id: i64,

    /// Vector of the tag ids to add to the sleep
    pub tag_ids: Vec<i64>
}

/// Graphql input fir adding a comment to a sleep
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct AddCommentToSleepInput {
    /// id of the sleep to add the comment to
    pub sleep_id: i64,

    /// text comment to add to the sleep
    pub comment: String
}

/// Graphql input to update a sleep value
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct UpdateSleepInput {
    /// id of the sleep to update
    pub sleep_id: i64,

    /// Optionally update the amount of sleep
    pub amount: Option<f64>,

    /// Optionally update the quality of the sleep
    pub quality: Option<i64>,
}

/// Graphql input to update a tag
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct UpdateTagInput {
    /// id of the tag to update
    pub tag_id: i64,

    /// Optionally update the name of the tag
    pub name: Option<String>,

    /// Optionally update rgb color value for the tag ex: Red (0xFF0000) is 16711680
    pub color: Option<i64>,
}

/// Graphql input to update a comment
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct UpdateCommentInput {
    /// id of the comment to update
    pub comment_id: i64,

    /// text to update the comment to
    pub comment: String
}

/// Graphql input for removing a tag from a sleep
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct RemoveTagFromSleepInput {
    /// id of the sleep to remove the tag from
    pub sleep_id: i64,

    /// the id of the tag to remove
    pub tag_id: i64
}

/// Graphql input to query sleeps in a given month
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct SleepsByMonthInput {
    /// month to query
    pub month: u8,

    /// year the month is in
    pub year: u16
}

/// Graphql input to to query sleeps in a given date range
#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct SleepsInRangeInput {
    /// month at either the start or end of the range
    pub month: u8,

    /// year at either the start or end of the range
    pub year: u16,

    /// Optionally include a specific day at the start or end of the range
    pub day: Option<u8>
}