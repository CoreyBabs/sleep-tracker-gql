use async_graphql::{Context, Object, SimpleObject, InputObject};
use crate::db_manager::DbmSleep;
use crate::DBManager;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sleep {
    pub id: i64,
    pub night: Night,
    pub amount: f64,
    pub quality: i64,
    pub tags: Option<Vec<Tag>>,
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
        match sleep {
            Some(s) => Some(Sleep::from_db(&s)),
            None => None,
        } 
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

#[derive(Debug, Clone, Default, PartialEq, SimpleObject)]
pub struct Tag {
    pub id: i64,
    pub name: String,
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

#[derive(Debug, Clone, Default, PartialEq, SimpleObject)]
pub struct Comment {
    pub id: i64,
    pub sleep_id: i64,
    pub comment: String,
}

impl Comment {
    pub async fn from_comment_id(dbm: &DBManager, comment_id: i64) -> Option<Comment> {
        let comment = dbm.get_comment(comment_id).await;
        match comment {
            Some(c) => Some(Comment {id: c.id, sleep_id: c.sleep_id, comment: c.comment.clone()}),
            None => None
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, SimpleObject)]
pub struct Night {
    pub day: u8,
    pub month: u8,
    pub year: u16,
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
}

#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct SleepInput {
    pub night: String,
    pub amount: f64,
    pub quality: i64,
    pub tags: Option<Vec<i64>>,
    pub comments: Option<Vec<String>>
}

#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct TagInput {
    pub name: String,
    pub color: i64,
}

#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct AddTagsToSleepInput {
    pub sleep_id: i64,
    pub tag_ids: Vec<i64>
}

#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct AddCommentToSleepInput {
    pub sleep_id: i64,
    pub comment: String
}

#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct UpdateSleepInput {
    pub sleep_id: i64,
    pub amount: Option<f64>,
    pub quality: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct UpdateTagInput {
    pub tag_id: i64,
    pub name: Option<String>,
    pub color: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct UpdateCommentInput {
    pub comment_id: i64,
    pub comment: String
}

#[derive(Debug, Clone, Default, PartialEq, InputObject)]
pub struct RemoveTagFromSleepInput {
    pub sleep_id: i64,
    pub tag_id: i64
}