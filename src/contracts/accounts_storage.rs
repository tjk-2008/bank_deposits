use crate::entities::Account;
use std::future::Future;
use uuid::Uuid;

pub trait AccountsStorage {
    fn add(&self, account: Account)
        -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Account>, Box<dyn std::error::Error>>>;

    fn remove(&self, uuid: Uuid) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(
        &self,
        account: &Account,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
