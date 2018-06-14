use resources::tv_show::*;
use super::MetadataProvider;

use tvmaze::client::Client;
use tvmaze::error::Error as TVMazeError;
use tvmaze::resources::{
    TVShow as TVMazeShow,
    TVShowSeason as TVMazeSeason,
    Episode as TVMazeEpisode,
};

use futures::Future;

#[derive(Clone)]
pub struct TVMazeProvider {
    client: Client,
}

impl TVMazeProvider {
    pub fn new() -> TVMazeProvider {
        TVMazeProvider {
            // This should never panic...
            client: Client::new().unwrap(),
        }
    }
}

impl MetadataProvider for TVMazeProvider {
    type ProviderShow = TVMazeShow;
    type ProviderSeason = TVMazeSeason;
    type ProviderEpisode = TVMazeEpisode;
    type ProviderError = TVMazeError;

    fn get_show(&self, id: &str)
        -> Result<Box<Future<Item = TVShow, Error = Self::ProviderError>>, Self::ProviderError>
    {
        Ok(Box::new(self
            .client
            .show_main_info(id)?
            .map(|r| Self::to_unify_show(&r))
        ))
    }

    fn get_show_seasons(&self, id: &str)
        -> Result<Box<Future<Item = Vec<TVShowSeason>, Error = Self::ProviderError>>, Self::ProviderError>
    {
        Ok(Box::new(self
            .client
            .show_seasons(id)?
            .map(|s| s.iter().map(|s| Self::to_unify_season(&s)).collect())
        ))
    }

    fn get_season_episodes(&self, _show_id: &str, season_id: &str)
        -> Result<Box<Future<Item = Vec<TVShowEpisode>, Error = Self::ProviderError>>, Self::ProviderError>
    {
        Ok(Box::new(self
            .client
            .season_episodes(season_id)?
            .map(|e| e.iter().map(|e| Self::to_unify_episode(&e)).collect())
        ))
    }

    fn search(&self, q: &str)
        -> Result<Box<Future<Item = Vec<TVShow>, Error = Self::ProviderError>>, Self::ProviderError>
    {
        Ok(Box::new(self
            .client
            .search_shows(q)?
            .map(|r| {
                r
                    .iter()
                    .map(|r| Self::to_unify_show(&r.show))
                    .collect()
            })
        ))
    }

    fn to_unify_show(p: &Self::ProviderShow) -> TVShow {
        TVShow {
            id: -1,
            name: p.name.clone(),
            summary: p.summary.clone(),
            cover_img: p.image.clone().map(|i| i.original),
            provider: TVMetadataProvider::TVMaze,
            provider_id: p.id.to_string(),
            provider_url: p.url.clone(),
            seasons: None,
        }
    }

    fn to_unify_season(p: &Self::ProviderSeason) -> TVShowSeason {
        TVShowSeason {
            id: -1,
            show_id: -1,
            num: p.number as i32,
            name: p.name.clone(),
            summary: p.summary.clone(),
            cover_img: p.image.clone().map(|i| i.original),
            provider_id: p.id.to_string(),
            provider_url: p.url.clone(),
            episodes: None,
        }
    }

    fn to_unify_episode(p: &Self::ProviderEpisode) -> TVShowEpisode {
        TVShowEpisode {
            id: -1,
            show_id: -1,
            season_id: -1,
            num: p.number.map(|x| x as i32),
            season_num: p.season as i32,
            name: p.name.clone(),
            summary: p.summary.clone(),
            cover_img: p.image.clone().map(|i| i.original),
            runtime: p.runtime.map(|x| x as i32),
            provider_id: p.id.to_string(),
            provider_url: p.url.clone(),
        }
    }
}
