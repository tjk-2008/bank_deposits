use std::sync::Arc;

use async_sqlite::{JournalMode, PoolBuilder};
use ohkami::fang::Context;
use ohkami::{Ohkami, Route};

mod context;
mod contracts;
mod contracts_impl;
mod entities;
mod http;

use context::RequestContext;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        let pool = PoolBuilder::new()
            .path("database/data.db")
            .journal_mode(JournalMode::Wal)
            .num_conns(10)
            .open()
            .await
            .expect("Database creation error");

        let request_context = RequestContext::new(Arc::new(pool));

        Ohkami::new((
            Context::new(request_context),
            "/deposits".POST(http::deposits::add_deposit),
            "/deposits/:id"
                .GET(http::deposits::get_deposit)
                .PUT(http::deposits::update_deposit)
                .DELETE(http::deposits::remove_deposit),
            "/clients".POST(http::clients::add_client),
            "/clients/:id"
                .GET(http::clients::get_client)
                .PUT(http::clients::update_client)
                .DELETE(http::clients::remove_client),
            "/accounts".POST(http::accounts::add_account),
            "/accounts/:id"
                .GET(http::accounts::get_account)
                .PUT(http::accounts::update_account)
                .DELETE(http::accounts::remove_account),
        ))
        .howl("localhost:3000")
        .await;

        Ok(())
    })
}
