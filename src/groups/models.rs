//! Models for groups
//!
//! Groups are stored in the `groups` table where each row is a group member
//! or a meeting 

use chrono::NaiveDateTime;

use crate::models::Attendable;
use crate::models::User;
use crate::schema::*;

/// Models a group in the database
#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub location: Option<String>,
}

/// Used to create a new group in the database
#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
    pub owner_id: i32,
    pub location: Option<String>,
}

/// Models a meeting in the database
#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Associations, Serialize)]
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
    fn group_id(&self) -> Option<i32> {
        if !self.is_event() {
            Some(self.group_id)
        } else {
            None
        }
    }
    fn is_event(&self) -> bool {
        false
    }
    fn url(&self) -> String {
        format!("/h/{}", self.group_id)
    }
}

/// Used to create a new meeting in the database
#[derive(Debug, Default, Clone, FromForm, Insertable)]
#[table_name = "meetings"]
pub struct NewMeeting {
    pub code: String,
    pub group_id: i32,
}

/// Models the identification info of a group member
#[derive(Debug, PartialEq, Clone, Queryable, Associations, Identifiable)]
#[belongs_to(Group)]
#[belongs_to(User)]
#[table_name = "relation_group_user"]
pub struct RelationGroupUser {
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
}

/// Used to create a new ID number for a group member
#[derive(Debug, Default, Clone, Insertable)]
#[table_name = "relation_group_user"]
pub struct NewRelationGroupUser {
    pub group_id: i32,
    pub user_id: i32,
}
