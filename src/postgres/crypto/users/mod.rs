pub mod model;
pub use model::*;

#[derive(Debug)]
pub struct UserInfo {
    pub username: String,
    pub sig_scheme: String,
    pub keypair: String,
    pub pubkey: String,
}

pub struct UsersData;

#[async_trait::async_trait]
impl crate::UsersManager for UsersData {
    type User = UserInfo;
    type Error = anyhow::Error;

    async fn get_all(&self) -> Result<Vec<Self::User>, anyhow::Error> {
        let users = model::User::get_all()?
            .iter()
            .map(|user| UserInfo::from(user.clone()))
            .collect();
        Ok(users)
    }

    async fn find(&self, username: String) -> Result<Self::User, anyhow::Error> {
        let user = model::User::find(username)?;
        Ok(UserInfo::from(user))
    }

    async fn delete(&self, username: String) -> Result<(), anyhow::Error> {
        model::User::delete(username)?;
        Ok(())
    }

    async fn update(&self, user: Self::User) -> Result<(), anyhow::Error> {
        model::User::update(model::UserForm::from(user))?;
        Ok(())
    }

    async fn insert(&self, user: Self::User) -> Result<(), anyhow::Error> {
        model::User::insert(model::UserForm::from(user))?;
        Ok(())
    }
}
