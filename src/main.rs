use std::sync::Arc;

use ohkami::fang::Context;
use ohkami::{Ohkami, Route};

mod context;
mod contracts;
mod contracts_impl;
mod entities;
mod http;

use context::RequestContext;
use contracts_impl::ManagedConnectionPool;

const DATABASE_PATH: &str = "database/data.db";
const MAX_CONNECTIONS_COUNT: usize = 10;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        let connection_pool =
            ManagedConnectionPool::initialize(DATABASE_PATH.to_string(), MAX_CONNECTIONS_COUNT)
                .await;

        let request_context = RequestContext::new(Arc::new(connection_pool));

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
