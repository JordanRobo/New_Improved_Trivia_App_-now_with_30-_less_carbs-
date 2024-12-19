<script lang="ts">
	import { Minus, Plus } from "svelte-radix";
	import { data } from "./test";

	let score = $state(5);
	let player_name = $state("");
	const points = [5, 10, 15, 20];

	function addPoints() {
		let player = data.players.find((player) => player.username === player_name);
		let new_score = player!.score + score;
		console.log(new_score);
	}

	function subtractPoints() {
		let player = data.players.find((player) => player.username === player_name);
		let new_score = player!.score - score;
		console.log(new_score);
	}
</script>

<div class="w-screen p-2">
	<table class="table w-full">
		<!-- head -->
		<thead>
			<tr>
				<th></th>
				<th>Player</th>
				<th class="text-right">Score</th>
			</tr>
		</thead>
		<tbody>
			<!-- rows -->
			{#each data.players as player}
				<tr>
					<th>
						<label>
							<input bind:group={player_name} type="radio" class="radio" value={player.username} />
						</label>
					</th>
					<td>
						<div class="flex items-center gap-3">
							<div class="avatar">
								<div class="mask mask-squircle h-8 w-8">
									<img src={player.avatar} alt="player avatar" />
								</div>
							</div>
							<div>
								<div class="font-bold">{player.username}</div>
							</div>
						</div>
					</td>
					<td class="text-right font-bold">{player.score}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<div class="space-y-2">
	<div class="flex justify-center">
		<div class="join">
			<button class="btn btn-primary join-item" onclick={() => subtractPoints()}><Minus /></button>
			<select bind:value={score} class="select select-bordered w-full max-w-xs join-item">
				{#each points as point}
					<option value={point}>{point}</option>
				{/each}
			</select>
			<button class="btn btn-secondary join-item" onclick={() => addPoints()}><Plus /></button>
		</div>
	</div>
	<div class="flex justify-center">
		<div class="join">
			<button class="btn btn-primary join-item">Button</button>
			<button class="btn join-item">Button</button>
			<button class="btn btn-secondary join-item">Button</button>
		</div>
	</div>
</div>
