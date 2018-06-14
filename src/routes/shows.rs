use db::tv_show::*;
use error::WebError;
use providers::metadata::MetadataProvider;
use resources::tv_show::*;
use AppState;

use actix_web::{AsyncResponder, Form, HttpResponse, Path, ResponseError, State};
use futures::{future, Future};

#[derive(Deserialize)]
pub struct AddShow {
    id: String,
}

#[derive(Deserialize)]
pub struct GetShow {
    id: i32,
}

pub fn shows_get(
    (state, req): (State<AppState>, Path<GetShow>),
) -> Box<Future<Item = HttpResponse, Error = WebError>> {
    Box::new(
        get_entire_show(req.id, state.db_pool.clone()).and_then(|show| match show {
            Ok(show) => Ok(HttpResponse::Ok().json(show)),
            Err(e) => Ok(e.error_response()),
        }),
    )
}

pub fn shows_all(state: State<AppState>) -> Box<Future<Item = HttpResponse, Error = WebError>> {
    Box::new(
        get_all_shows(state.db_pool.clone())
            .and_then(|shows| match shows {
                Ok(shows) => Ok(HttpResponse::Ok().json(shows)),
                Err(e) => Ok(e.error_response()),
            })
            .responder(),
    )
}

pub fn shows_add(
    (state, form): (State<AppState>, Form<AddShow>),
) -> Box<Future<Item = HttpResponse, Error = WebError>> {
    let provider = state.provider.clone();
    let load_show = provider.get_show(&form.id).unwrap().map_err(WebError::from);

    let provider = state.provider.clone();
    let load_seasons = load_show.and_then(move |show| {
        provider
            .get_show_seasons(&show.provider_id)
            .unwrap()
            .map(|seasons| (show, seasons))
            .map_err(WebError::from)
    });

    let provider = state.provider.clone();
    let load_eps = load_seasons.and_then(move |(mut show, seasons)| {
        let mut fut: Box<Future<Item = Vec<TVShowSeason>, Error = WebError>> =
            Box::new(future::ok(vec![]));

        for mut season in seasons.into_iter() {
            let provider = provider.clone();

            fut = Box::new(fut.and_then(move |mut seasons| {
                provider
                    .get_season_episodes("", &season.provider_id)
                    .unwrap()
                    .map(move |episodes| {
                        season.episodes = Some(episodes);
                        seasons.push(season);
                        seasons
                    })
                    .map_err(WebError::from)
            }));
        }

        fut.map(move |seasons| {
            show.seasons = Some(seasons);
            show
        })
    });

    let pool = state.db_pool.clone();

    Box::new(load_eps.and_then(move |show| {
        insert_show(&show, pool).and_then(|show| Ok(HttpResponse::Ok().json(show)))
    }))
}
