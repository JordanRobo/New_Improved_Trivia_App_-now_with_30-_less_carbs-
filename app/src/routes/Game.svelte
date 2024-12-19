<script lang="ts">
	import Player from "../lib/game/Player.svelte";
	import Host from "../lib/game/Host.svelte";

	let host = true;
</script>

{#if !host}
	<Player />
{:else}
	<Host />
{/if}
