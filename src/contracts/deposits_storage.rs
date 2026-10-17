use crate::entities::Deposit;
use std::future::Future;
use uuid::Uuid;

pub trait DepositsStorage {
    fn add(&self, deposit: Deposit)
        -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn get(
        &self,
        uuid: Uuid,
    ) -> impl Future<Output = Result<Option<Deposit>, Box<dyn std::error::Error>>>;

    fn remove(&self, uuid: Uuid) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn update(
        &self,
        deposit: &Deposit,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
