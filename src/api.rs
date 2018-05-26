use resources::{Person, TVShow};

pub const TVMAZE_BASE_URL: &'static str = "http://api.tvmaze.com";

#[derive(Debug, Deserialize)]
pub struct ShowSearchResult {
    score: f64,
    show: TVShow,
}

#[derive(Debug, Deserialize)]
pub struct PeopleSearchResult {
    score: f64,
    person: Person,
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
