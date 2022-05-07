#[async_trait::async_trait]
pub trait UsersManager {
    type User;
    type Error;

    async fn get_all(&self) -> Result<Vec<Self::User>, Self::Error>;
    async fn find(&self, username: String) -> Result<Self::User, Self::Error>;
    async fn delete(&self, username: String) -> Result<(), Self::Error>;
    async fn update(&self, user: Self::User) -> Result<(), Self::Error>;
    async fn insert(&self, user: Self::User) -> Result<(), Self::Error>;
}
