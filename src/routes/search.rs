use error::WebError;
use providers::metadata::MetadataProvider;
use AppState;

use actix_web::{AsyncResponder, HttpResponse, Query, State};
use futures::Future;

#[derive(Deserialize)]
pub struct Search {
    q: String,
}

pub fn search(
    (state, search_q): (State<AppState>, Query<Search>),
) -> Box<Future<Item = HttpResponse, Error = WebError>> {
    Box::new(
        state
            .provider
            .search(&search_q.q)
            .unwrap()
            .and_then(|results| Ok(HttpResponse::Ok().json(&results)))
            .map_err(WebError::from)
            .responder(),
    )
}
