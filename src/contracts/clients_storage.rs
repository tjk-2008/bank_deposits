use crate::entities::Client;
use std::future::Future;
use uuid::Uuid;

pub trait ClientsStorage {
    fn add(&self, client: Client) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Client>, Box<dyn std::error::Error>>>;

    fn remove(&self, uuid: Uuid) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(
        &self,
        client: &Client,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
