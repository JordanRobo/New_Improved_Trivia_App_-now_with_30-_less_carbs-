export type User = {
	user_id: string;
	username: string;
	avatar: string | null;
};

export type Game = {
	game_id: string;
	host_id: string;
	status: number;
};

export type PlayerScores = {
	game_id: string;
	player_id: string;
	score: number;
};
