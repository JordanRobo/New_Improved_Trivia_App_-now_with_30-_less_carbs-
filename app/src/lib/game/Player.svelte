<script lang="ts">
	let player = {
		name: "John Doe",
		score: 20,
		avatar: "http://localhost:8080/avatars/bmo.jpg",
	};

	// onMount check if player is part of game
	// SQL = playerscores table
</script>

<!-- View if player is part of a game -->
<div class="card glass w-80">
	<div class="card-body items-center text-center">
		<div class="avatar mb-2">
			<div class="ring-primary ring-offset-base-100 w-32 rounded-full ring ring-offset-2 -mt-28">
				<img src={player.avatar} alt="player avatar" />
			</div>
		</div>
		<h2 class="card-title">{player.name}</h2>
		<p class="text-4xl font-bold">{player.score}</p>
		<div class="card-actions justify-end">
			<button class="btn btn-primary w-60">Buzzer</button>
		</div>
	</div>
</div>
