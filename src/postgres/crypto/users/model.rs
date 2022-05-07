use crate::postgres::schema::users;
use crate::postgres::{crypto::users::UserInfo, db};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct UserForm {
    pub username: String,
    pub sig_scheme: String,
    pub keypair: String,
    pub pubkey: String,
}

impl From<UserInfo> for UserForm {
    fn from(user: UserInfo) -> Self {
        Self {
            username: user.username,
            sig_scheme: user.sig_scheme,
            keypair: user.keypair,
            pubkey: user.pubkey,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub sig_scheme: String,
    pub keypair: String,
    pub pubkey: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            sig_scheme: user.sig_scheme,
            keypair: user.keypair,
            pubkey: user.pubkey,
        }
    }
}

impl User {
    pub fn get_all() -> anyhow::Result<Vec<Self>> {
        let conn = db::connection()?;
        let users = users::table.load::<User>(&conn)?;
        Ok(users)
    }

    pub fn find(username: String) -> anyhow::Result<Self> {
        let conn = db::connection()?;
        let user = users::table
            .filter(users::username.eq(username))
            .first(&conn)?;
        Ok(user)
    }

    pub fn delete(username: String) -> anyhow::Result<usize> {
        let conn = db::connection()?;
        let res =
            diesel::delete(users::table.filter(users::username.eq(username))).execute(&conn)?;
        Ok(res)
    }

    pub fn update(user: UserForm) -> anyhow::Result<Self> {
        let conn = db::connection()?;
        let user = diesel::update(users::table)
            .filter(users::username.eq(user.username.clone()))
            .set(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn insert(user: UserForm) -> anyhow::Result<Self> {
        let conn = db::connection()?;
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }
}
