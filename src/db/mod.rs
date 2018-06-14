use std::ptr::NonNull;

use error::WebError;

use actix::prelude::*;
use postgres::rows::Rows;
use postgres::types::ToSql;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

#[macro_export]
macro_rules! db_params {
    ($($value:expr),+) => {
        vec![
            $(
                ToSqlWrapper::new(Box::new($value.clone()) as Box<ToSql + Send>),
            )+
        ]
    };
}

pub mod tv_show;

/// A thread-safe wrapper around the ToSql trait.
///
/// This type is necessary to pass SQL parameters to the database
/// query types (such as `DBQuery` and `DBInsertMany`). `ToSqlWrapper`s
/// are constructed from trait objects (a `Box`) that implement `ToSql` and
/// `Send`. It turns the `Box` into a pointer and allows `&ToSql` references
/// to the pointer's memory.
pub struct ToSqlWrapper {
    inner: NonNull<ToSql>,
}

unsafe impl Send for ToSqlWrapper {}

impl ToSqlWrapper {
    pub fn new(t: Box<ToSql + Send>) -> ToSqlWrapper {
        ToSqlWrapper {
            // I'm wary about unwrapping here, but it should be okay since
            // `ToSqlWrapper`s are only created using the `db_params!` macro
            // in the code.
            inner: NonNull::new(Box::into_raw(t) as *mut ToSql).unwrap(),
        }
    }

    fn get(&self) -> &ToSql {
        unsafe { self.inner.as_ref() }
    }
}

impl Drop for ToSqlWrapper {
    fn drop(&mut self) {
        let ptr = self.inner.as_ptr();
        unsafe {
            Box::from_raw(ptr);
        }
    }
}

pub struct DBExecutor {
    pool: Pool<PostgresConnectionManager>,
}

pub trait DBInsertable {
    fn as_db_params(&self) -> Vec<ToSqlWrapper>;
}

pub struct DBQuery {
    query: String,
    params: Vec<ToSqlWrapper>,
}

pub struct DBInsertMany {
    query: &'static str,
    returning: &'static str,
    values: u8,
    params: Vec<Vec<ToSqlWrapper>>,
}

impl DBExecutor {
    pub fn new(pool: Pool<PostgresConnectionManager>) -> DBExecutor {
        DBExecutor { pool }
    }

    pub fn pool_get(&mut self) -> Result<PooledConnection<PostgresConnectionManager>, WebError> {
        self.pool.get().map_err(WebError::from)
    }
}

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<DBQuery> for DBExecutor {
    type Result = Result<Rows, WebError>;

    fn handle(&mut self, msg: DBQuery, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool_get()?;
        let params: Vec<&ToSql> = msg.params.iter().map(|p| p.get()).collect();
        conn.query(&msg.query, params.as_slice())
            .map_err(WebError::from)
    }
}

impl Handler<DBInsertMany> for DBExecutor {
    type Result = Result<Rows, WebError>;

    fn handle(&mut self, msg: DBInsertMany, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool_get()?;
        let mut query = msg.query.to_string();
        let mut param_num = 1;

        // Construct query parameters string
        for i in 0..msg.params.len() {
            let params: Vec<String> = (0..msg.values)
                .map(|_| {
                    let param = format!("${}", param_num);
                    param_num += 1;
                    param
                })
                .collect();

            query.push_str(" (");
            query += &params.join(",");
            query.push(')');

            if i != msg.params.len() - 1 {
                query.push(',');
            }
        }

        query += msg.returning;

        let params: Vec<&ToSql> = msg.params.iter().flatten().map(|p| p.get()).collect();
        conn.query(&query, params.as_slice())
            .map_err(WebError::from)
    }
}

impl DBQuery {
    pub fn new(query: String, params: Vec<ToSqlWrapper>) -> DBQuery {
        DBQuery { query, params }
    }
}

impl Message for DBQuery {
    type Result = Result<Rows, WebError>;
}

impl Message for DBInsertMany {
    type Result = Result<Rows, WebError>;
}

impl DBInsertMany {
    pub fn new(
        query: &'static str,
        returning: &'static str,
        values: u8,
        params: Vec<Vec<ToSqlWrapper>>,
    ) -> DBInsertMany {
        DBInsertMany {
            query,
            returning,
            values,
            params,
        }
    }
}
