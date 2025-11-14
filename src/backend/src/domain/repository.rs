use anyhow::Result;
use async_trait::async_trait;

use crate::domain::server::{NewServerInstance, ServerInstance};
use crate::domain::user::{NewUser, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(&self, id: i64) -> Result<Option<User>>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn create(&self, user: NewUser) -> Result<User>;
    async fn list_all(&self) -> Result<Vec<User>>;
    async fn update(&self, user: User) -> Result<User>;
    async fn timestamp_login(&self, id: i64) -> Result<()>;
    async fn delete(&self, id: i64) -> Result<()>;
}

#[async_trait]
pub trait ServerRepository: Send + Sync {
    async fn get_by_id(&self, id: i64) -> Result<Option<ServerInstance>>;
    async fn get_by_name(&self, name: &str) -> Result<Option<ServerInstance>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<ServerInstance>>;
    async fn create(&self, server_instance: NewServerInstance) -> Result<ServerInstance>;
    async fn list_all(&self) -> Result<Vec<ServerInstance>>;
    async fn update(&self, server_instance: ServerInstance) -> Result<ServerInstance>;
    async fn timestamp_start(&self, id: i64) -> Result<()>;
    async fn delete(&self, id: i64) -> Result<()>;
}
