<script lang="ts">
	function openModal() {
		//@ts-ignore
		join_modal.showModal();
	}

	const getUser = async () => {
		const userData = localStorage.getItem("userData");
		if (!userData) {
			window.location.hash = "/login";
		} else {
			const player = JSON.parse(userData);
			return player;
		}
	};

	async function startGame(host_id: string) {}
</script>

{#await getUser()}
	<span class="loading loading-spinner text-primary"></span>
{:then player}
	<div class="card glass w-screen rounded-[999px] absolute h-96 inset-x-0 bottom-0 -mb-32">
		<div class="card-body items-center text-center relative">
			{#if !player?.avatar}
				<div class="avatar placeholder mb-2">
					<div class="bg-neutral text-neutral-content w-32 rounded-full -mt-28">
						<span class="text-3xl">{player?.username[0].toUpperCase()}</span>
					</div>
				</div>
			{:else}
				<div class="avatar mb-2">
					<div class="ring-primary ring-offset-base-100 w-32 rounded-full ring ring-offset-2 -mt-28">
						<img src={player.avatar} alt="player avatar" />
					</div>
				</div>
			{/if}

			<h2 class="card-title text-4xl">{player?.username}</h2>
			<div class="card-actions justify-end">
				<button onclick={() => openModal()} class="btn btn-primary">Join Game</button>
				<button onclick={() => startGame(player.user_id)} class="btn btn-secondary">Start Game</button>
			</div>
		</div>
	</div>
{:catch error}
	<p>Something went wrong</p>
{/await}
