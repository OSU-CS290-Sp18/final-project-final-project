use resources::tv_show::*;

use futures::Future;

pub mod tvmaze;

pub use self::tvmaze::TVMazeProvider;

pub trait MetadataProvider: Clone {
    type ProviderShow;
    type ProviderSeason;
    type ProviderEpisode;
    type ProviderError;

    fn get_show(&self, id: &str)
        -> Result<Box<Future<Item = TVShow, Error = Self::ProviderError>>, Self::ProviderError>;
    fn get_show_seasons(&self, id: &str)
        -> Result<Box<Future<Item = Vec<TVShowSeason>, Error = Self::ProviderError>>, Self::ProviderError>;
    fn get_season_episodes(&self, show_id: &str, season_id: &str)
        -> Result<Box<Future<Item = Vec<TVShowEpisode>, Error = Self::ProviderError>>, Self::ProviderError>;

    fn search(&self, q: &str)
        -> Result<Box<Future<Item = Vec<TVShow>, Error = Self::ProviderError>>, Self::ProviderError>;

    fn to_unify_show(p: &Self::ProviderShow) -> TVShow;
    fn to_unify_season(p: &Self::ProviderSeason) -> TVShowSeason;
    fn to_unify_episode(p: &Self::ProviderEpisode) -> TVShowEpisode;
}
