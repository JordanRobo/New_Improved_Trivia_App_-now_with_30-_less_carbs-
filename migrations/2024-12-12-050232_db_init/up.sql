-- Your SQL goes here
CREATE TABLE users (
	user_id TEXT NOT NULL PRIMARY KEY,
	username TEXT NOT NULL UNIQUE,
	pass TEXT NOT NULL,
	avatar TEXT
);

CREATE TABLE games (
	game_id TEXT NOT NULL PRIMARY KEY,
	host_id TEXT NOT NULL,
	status INTEGER DEFAULT 0,
	FOREIGN KEY (host_id) REFERENCES Users (user_id)
);

CREATE TABLE player_scores (
	game_id TEXT NOT NULL,
	player_id TEXT NOT NULL,
	score INTEGER DEFAULT 0,
	PRIMARY KEY (game_id, player_id),
	FOREIGN KEY (game_id) REFERENCES Games (game_id),
	FOREIGN KEY (player_id) REFERENCES Users (user_id)
);
