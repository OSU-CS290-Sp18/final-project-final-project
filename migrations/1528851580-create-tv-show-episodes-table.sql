CREATE TABLE tv_show_episodes (
  id SERIAL NOT NULL,
  show_id INT NOT NULL,
  season_id INT NOT NULL,
  num INT,
  season_num INT NOT NULL,
  name TEXT NOT NULL,
  summary TEXT,
  cover_img TEXT,
  runtime INT,
  provider_id TEXT NOT NULL,
  provider_url TEXT NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (show_id) REFERENCES tv_shows (id) ON DELETE CASCADE,
  FOREIGN KEY (season_id) REFERENCES tv_show_seasons (id) ON DELETE CASCADE
);
