let game = $state({
	game_id: "",
});

export function getGame() {
	return game;
}

export function setGameId(input: string) {
	game.game_id = input;
}
