use chrono::NaiveDateTime;

use crate::models::Attendable;
use crate::schema::*;
use crate::users::models::User;

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub location: Option<String>,
}

#[derive(Debug, Default, FromForm, Insertable, AsChangeset)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
    pub owner_id: i32,
    pub location: Option<String>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Group)]
pub struct Meeting {
    pub id: i32,
    pub happened_at: NaiveDateTime,
    pub code: String,
    pub group_id: i32,
    pub hosted_by: i32,
}

impl Attendable for Meeting {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        format!("Meeting at: {}", self.happened_at)
    }
    fn time(&self) -> NaiveDateTime {
        self.happened_at
    }
    fn code(&self) -> String {
        self.code.clone()
    }
    fn owner_id(&self) -> i32 {
        self.hosted_by
    }
    fn is_event(&self) -> bool {
        false
    }
    fn url(&self) -> String {
        format!("/h/{}", self.group_id)
    }
}

#[derive(Debug, Default, FromForm, Insertable)]
#[table_name = "meetings"]
pub struct NewMeeting {
    pub code: String,
    pub group_id: i32,
}

#[derive(Debug, PartialEq, Queryable, Associations, Identifiable)]
#[belongs_to(Group)]
#[belongs_to(User)]
#[table_name = "relation_group_user"]
pub struct RelationGroupUser {
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "relation_group_user"]
pub struct NewRelationGroupUser {
    pub group_id: i32,
    pub user_id: i32,
}