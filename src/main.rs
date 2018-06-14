#![feature(iterator_flatten)]

extern crate actix;
extern crate actix_web;
extern crate futures;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tvmaze;

#[macro_use]
mod db;
mod error;
mod providers;
mod resources;
mod routes;

use std::env;

use db::DBExecutor;
use providers::metadata::TVMazeProvider;
use routes::*;

use actix::prelude::*;
use actix_web::{http, server, App};
use r2d2::Pool as DBPool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

pub struct AppState {
    provider: TVMazeProvider,
    db_pool: Addr<Syn, DBExecutor>,
}

fn main() {
    let pg_conn = env::var("PG_URL").expect("Unable to get PG_URL environment variable.");
    let sys = System::new("Unify");
    let manager = PostgresConnectionManager::new(pg_conn, TlsMode::None)
        .expect("Unable to connect to PostgreSQL server.");
    let db_pool = DBPool::new(manager).expect("Unable to create database pool.");
    let addr = SyncArbiter::start(4, move || DBExecutor::new(db_pool.clone()));

    server::new(move || {
        let state = AppState {
            db_pool: addr.clone(),
            provider: TVMazeProvider::new(),
        };

        App::with_state(state)
            .scope("/api", |api_scope| {
                api_scope
                    .nested("/shows", |scope| {
                        scope
                            .resource("", |r| {
                                r.get().with(shows_all);
                                r.post().with(shows_add);
                            })
                            .resource("/{id}", |r| {
                                r.get().with(shows_get);
                            })
                    })
                    .route("/search", http::Method::GET, search)
            })
    }).bind("0.0.0.0:8080").expect("Unable to create HTTP server.").start();

    println!("Listening on 8080.");

    let _ = sys.run();
}
