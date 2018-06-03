use resources::{Person, TVShow};

#[cfg(not(feature = "tls"))]
pub const TVMAZE_BASE_URL: &'static str = "http://api.tvmaze.com";

#[cfg(feature = "tls")]
pub const TVMAZE_BASE_URL: &'static str = "https://api.tvmaze.com";

#[derive(Debug, Deserialize)]
pub struct ShowSearchResult {
    pub score: f64,
    pub show: TVShow,
}

#[derive(Debug, Deserialize)]
pub struct PeopleSearchResult {
    pub score: f64,
    pub person: Person,
}

#[derive(Debug)]
pub enum LinkEmbed<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

#[derive(Debug)]
pub enum ShowLookup<'a> {
    IMDB(&'a str),
    TheTVDB(&'a str),
    TVRage(&'a str),
}

impl<'a> ShowLookup<'a> {
    pub fn as_query_param(&self) -> (&str, &str) {
        match self {
            ShowLookup::IMDB(id) => ("imdb", id),
            ShowLookup::TheTVDB(id) => ("thetvdb", id),
            ShowLookup::TVRage(id) => ("tvrage", id),
        }
    }
}
