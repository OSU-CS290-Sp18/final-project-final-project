use std::collections::HashMap;

use error::WebError;
use resources::tv_show::*;
use super::*;

use futures::{future, Future};
use postgres::rows::{Row, Rows};
use postgres::types::ToSql;

pub const SELECT_JOIN_SHOW: &'static str =
    "SELECT A.id as show_id,
            A.name as show_name,
            A.summary as show_summary,
            A.cover_img as show_cover_img,
            A.provider as show_provider,
            A.provider_id as show_provider_id,
            A.provider_url as show_provider_url,
            B.id as season_id,
            B.show_id as season_show_id,
            B.num as season_num,
            B.name as season_name,
            B.summary as season_summary,
            B.cover_img as season_cover_img,
            B.provider_id as season_provider_id,
            B.provider_url as season_provider_url,
            C.id as episode_id,
            C.show_id as episode_show_id,
            C.season_id as episode_season_id,
            C.num as episode_num,
            C.season_num as episode_season_num,
            C.name as episode_name,
            C.summary as episode_summary,
            C.cover_img as episode_cover_img,
            C.runtime as episode_runtime,
            C.provider_id as episode_provider_id,
            C.provider_url as episode_provider_url
     FROM tv_shows A
     LEFT OUTER JOIN tv_show_seasons B on B.show_id = A.id
     LEFT OUTER JOIN tv_show_episodes C on C.show_id = A.id AND C.season_id = B.id ";

pub fn process_single_joined_show<'a>(rows: Vec<Row<'a>>) -> TVShow {
    let mut show = TVShow::from(&rows[0]);
    let mut show_seasons = HashMap::new();
    let mut episodes = HashMap::new();

    for row in rows.iter() {
        let season_id: i32 = row.get("season_id");

        if !show_seasons.contains_key(&season_id) {
            show_seasons.insert(season_id, TVShowSeason::from(row));
            episodes.insert(season_id, Vec::new());
        }
        episodes.get_mut(&season_id).unwrap().push(TVShowEpisode::from(row));
    }

    let mut seasons: Vec<TVShowSeason> = show_seasons.into_iter().map(|(_, s)| s).collect();

    for mut season in seasons.iter_mut() {
        let episodes = episodes.remove(&season.id).unwrap();
        season.episodes = Some(episodes);
    }

    show.seasons = Some(seasons);
    show
}

pub fn process_multiple_joined_shows(rows: Rows) -> Vec<TVShow> {
    rows
        .into_iter()
        .fold(HashMap::new(), |mut acc, r| {
            let show_id: i32 = r.get("show_id");
            if !acc.contains_key(&show_id) {
                let rows = vec![r];
                acc.insert(show_id, rows);
            } else {
                acc.get_mut(&show_id).unwrap().push(r);
            }
            acc
        })
        .into_iter()
        .map(|(_, rows)| process_single_joined_show(rows))
        .collect()
}

pub fn get_entire_show(id: i32, pool: Addr<Syn, DBExecutor>)
    -> Box<Future<Item = Result<TVShow, WebError>, Error = WebError>>
{
    Box::new(pool
        .send(DBQuery::new(
            [SELECT_JOIN_SHOW, " WHERE A.id = $1"].join(""),
            db_params![id]
        ))
        .map(move |rows| match rows {
            Ok(rows) => {
                if rows.len() == 0 {
                    Err(WebError::ResourceNotFound(id))
                } else {
                    Ok(process_single_joined_show(rows.into_iter().collect()))
                }
            }
            Err(e) => Err(e.into()),
        })
        .map_err(WebError::from))
}

pub fn get_all_shows(pool: Addr<Syn, DBExecutor>)
    -> Box<Future<Item = Result<Vec<TVShow>, WebError>, Error = WebError>>
{
    Box::new(pool
        .send(DBQuery::new(
            SELECT_JOIN_SHOW.into(),
            vec![],
        ))
        .map(|rows| match rows {
            Ok(rows) => {
                if rows.len() == 0 {
                    Ok(vec![])
                } else {
                    Ok(process_multiple_joined_shows(rows))
                }
            }
            Err(e) => Err(e.into()),
        })
        .map_err(WebError::from))
}

pub fn insert_show(show: &TVShow, pool: Addr<Syn, DBExecutor>)
    -> Box<Future<Item = TVShow, Error = WebError>>
{
    let insert_show = pool.send(DBQuery::new(
            "INSERT INTO tv_shows (
                name,
                summary,
                cover_img,
                provider,
                provider_id,
                provider_url
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id as show_id,
                      name as show_name,
                      summary as show_summary,
                      cover_img as show_cover_img,
                      provider as show_provider,
                      provider_id as show_provider_id,
                      provider_url as show_provider_url".into(),
            show.as_db_params(),
        ))
        .map(|rows| match rows {
            Ok(rows) => TVShow::from(&rows.get(0)),
            Err(e) => panic!("An ERROR: {:?}", e),
        })
        .map_err(WebError::from);

    if show.seasons.is_some() {
        let seasons = show.seasons.clone().unwrap();
        Box::new(insert_show.and_then(move |mut show| {
            let seasons = seasons
                .into_iter()
                .map(|mut s| {
                    s.show_id = show.id;
                    s
                })
                .collect();

            insert_seasons(seasons, pool)
                .map(move |seasons| {
                    show.seasons = Some(seasons);
                    show
                })
        }))
    } else {
        Box::new(insert_show)
    }
}

pub fn insert_seasons(seasons: Vec<TVShowSeason>, pool: Addr<Syn, DBExecutor>)
    -> Box<Future<Item = Vec<TVShowSeason>, Error = WebError>>
{
    let mut fut: Box<Future<Item = Vec<TVShowSeason>, Error = WebError>> = Box::new(future::ok(vec![]));

    for season in seasons.into_iter() {
        let pool = pool.clone();
        let pool_clone = pool.clone();
        let episodes = season.episodes.clone();
        let tmp = Box::new(fut.and_then(move |seasons| {
            pool.send(DBQuery::new(
                "INSERT INTO tv_show_seasons (
                    show_id,
                    num,
                    name,
                    summary,
                    cover_img,
                    provider_id,
                    provider_url
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id as season_id,
                          show_id as season_show_id,
                          num as season_num,
                          name as season_name,
                          summary as season_summary,
                          cover_img as season_cover_img,
                          provider_id as season_provider_id,
                          provider_url as season_provider_url".into(),
                season.as_db_params(),
            ))
            .map(|row| match row {
                Ok(row) => (seasons, TVShowSeason::from(&row.get(0))),
                Err(e) => panic!("An ERROR: {:?}", e),
            })
            .map_err(WebError::from)
        }));

        fut = if episodes.is_some() {
            let episodes = episodes.unwrap();
            Box::new(tmp.and_then(move |(mut seasons, mut season)| {
                let episodes = episodes
                    .into_iter()
                    .map(|mut e| {
                        e.show_id = season.show_id;
                        e.season_id = season.id;
                        e
                    })
                    .collect();

                insert_episodes(episodes, pool_clone)
                    .map(move |episodes| {
                        season.episodes = Some(episodes);
                        seasons.push(season);
                        seasons
                    })
            }))
        } else {
            Box::new(tmp.map(|(mut seasons, season)| {
                seasons.push(season);
                seasons
            }))
        };
    }

    fut
}

pub fn insert_episodes(episodes: Vec<TVShowEpisode>, pool: Addr<Syn, DBExecutor>)
    -> Box<Future<Item = Vec<TVShowEpisode>, Error = WebError>>
{
    let params = episodes.into_iter().map(|e| e.as_db_params()).collect();

    Box::new(pool
        .send(DBInsertMany::new(
            "INSERT INTO tv_show_episodes (
                show_id,
                season_id,
                num,
                season_num,
                name,
                summary,
                cover_img,
                runtime,
                provider_id,
                provider_url
            ) VALUES ",
            " RETURNING id as episode_id,
                        show_id as episode_show_id,
                        season_id as episode_season_id,
                        num as episode_num,
                        season_num as episode_season_num,
                        name as episode_name,
                        summary as episode_summary,
                        cover_img as episode_cover_img,
                        runtime as episode_runtime,
                        provider_id as episode_provider_id,
                        provider_url as episode_provider_url",
            10,
            params,
        ))
        .map(|rows| match rows {
            Ok(rows) => rows.into_iter().map(|r| TVShowEpisode::from(&r)).collect(),
            Err(e) => panic!("An ERROR: {:?}", e),
        })
        .map_err(WebError::from)
    )
}
