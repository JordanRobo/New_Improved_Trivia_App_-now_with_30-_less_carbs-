<script lang="ts">
	let game_id: string;

	function joinGame(gameId: string) {}
</script>

<dialog id="join_modal" class="modal">
	<div class="modal-box">
		<div class="label">
			<span class="label-text">Enter game join code</span>
		</div>
		<input type="text" class="input input-bordered w-full" />
		<button class="btn btn-primary w-full mt-2">Join</button>
	</div>
	<form method="dialog" class="modal-backdrop">
		<button>close</button>
	</form>
</dialog>
