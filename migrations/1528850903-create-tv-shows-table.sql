CREATE TYPE tv_metadata_provider AS ENUM (
  'tvmaze'
);

CREATE TABLE tv_shows (
  id SERIAL NOT NULL,
  name TEXT NOT NULL,
  summary TEXT,
  cover_img TEXT,
  provider tv_metadata_provider NOT NULL,
  provider_id TEXT NOT NULL,
  provider_url TEXT NOT NULL,
  PRIMARY KEY (id)
);
