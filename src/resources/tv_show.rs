use db::{DBInsertable, ToSqlWrapper};

use postgres::types::ToSql;
use postgres::rows::Row;

#[derive(Clone, Debug, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "tv_metadata_provider")]
pub enum TVMetadataProvider {
    #[postgres(name = "tvmaze")]
    TVMaze,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TVShow {
    pub id: i32,
    pub name: String,
    pub summary: Option<String>,
    pub cover_img: Option<String>,
    pub provider: TVMetadataProvider,
    pub provider_id: String,
    pub provider_url: String,
    pub seasons: Option<Vec<TVShowSeason>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TVShowSeason {
    pub id: i32,
    pub show_id: i32,
    pub num: i32,
    pub name: String,
    pub summary: Option<String>,
    pub cover_img: Option<String>,
    pub provider_id: String,
    pub provider_url: String,
    pub episodes: Option<Vec<TVShowEpisode>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TVShowEpisode {
    pub id: i32,
    pub show_id: i32,
    pub season_id: i32,
    pub num: Option<i32>,
    pub season_num: i32,
    pub name: String,
    pub summary: Option<String>,
    pub cover_img: Option<String>,
    pub runtime: Option<i32>,
    pub provider_id: String,
    pub provider_url: String,
}

impl<'a> From<&'a Row<'a>> for TVShow {
    fn from(row: &'a Row) -> Self {
        TVShow {
            id: row.get("show_id"),
            name: row.get("show_name"),
            summary: row.get("show_summary"),
            cover_img: row.get("show_cover_img"),
            provider: row.get("show_provider"),
            provider_id: row.get("show_provider_id"),
            provider_url: row.get("show_provider_url"),
            seasons: None,
        }
    }
}

impl DBInsertable for TVShow {
    fn as_db_params(&self) -> Vec<ToSqlWrapper> {
        db_params![
            self.name,
            self.summary,
            self.cover_img,
            self.provider,
            self.provider_id,
            self.provider_url
        ]
    }
}

impl<'a> From<&'a Row<'a>> for TVShowSeason {
    fn from(row: &'a Row) -> Self {
        TVShowSeason {
            id: row.get("season_id"),
            show_id: row.get("season_show_id"),
            num: row.get("season_num"),
            name: row.get("season_name"),
            summary: row.get("season_summary"),
            cover_img: row.get("season_cover_img"),
            provider_id: row.get("season_provider_id"),
            provider_url: row.get("season_provider_url"),
            episodes: None,
        }
    }
}

impl DBInsertable for TVShowSeason {
    fn as_db_params(&self) -> Vec<ToSqlWrapper> {
        db_params![
            self.show_id,
            self.num,
            self.name,
            self.summary,
            self.cover_img,
            self.provider_id,
            self.provider_url
        ]
    }
}

impl<'a> From<&'a Row<'a>> for TVShowEpisode {
    fn from(row: &'a Row) -> Self {
        TVShowEpisode {
            id: row.get("episode_id"),
            show_id: row.get("episode_show_id"),
            season_id: row.get("episode_season_id"),
            num: row.get("episode_num"),
            season_num: row.get("episode_season_num"),
            name: row.get("episode_name"),
            summary: row.get("episode_summary"),
            cover_img: row.get("episode_cover_img"),
            runtime: row.get("episode_runtime"),
            provider_id: row.get("episode_provider_id"),
            provider_url: row.get("episode_provider_url"),
        }
    }
}

impl DBInsertable for TVShowEpisode {
    fn as_db_params(&self) -> Vec<ToSqlWrapper> {
        db_params![
            self.show_id,
            self.season_id,
            self.num,
            self.season_num,
            self.name,
            self.summary,
            self.cover_img,
            self.runtime,
            self.provider_id,
            self.provider_url
        ]
    }
}
