<script lang="ts">
	import { MediaQuery } from "svelte/reactivity";
	import Router from "svelte-spa-router";
	import Home from "./routes/Home.svelte";
	import Login from "./routes/Login.svelte";
	import NavBar from "./lib/nav/NavBar.svelte";
	import Profile from "./routes/Profile.svelte";
	import Game from "./routes/Game.svelte";
	import JoinGame from "./lib/modal/JoinGame.svelte";
	import WsTest from "./routes/WSTest.svelte";

	const routes = {
		"/": Home,
		"/login": Login,
		"/profile": Profile,
		"/game": Game,
		"/ws-test": WsTest,
	};

	const mobile = new MediaQuery("max-width: 520px");
</script>

{#if mobile.current}
	<Router {routes} />

	<JoinGame />
	<NavBar />
{:else}
	<div class="text-center">
		<h1 class="text-4xl font-bold">This App is for mobile only</h1>
		<p class="text-2xl">Please reopen this link on your mobile device</p>
	</div>
{/if}
