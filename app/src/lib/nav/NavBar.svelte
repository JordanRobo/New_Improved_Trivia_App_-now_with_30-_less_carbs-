<script lang="ts">
	import { Person, Exit, Mix } from "svelte-radix";

	function navigate(url: string) {
		window.location.hash = url;
	}

	function logout() {
		localStorage.removeItem("userData");
		window.location.hash = "/login";
	}
</script>

<nav class="btm-nav">
	<button onclick={() => navigate("/game")}>
		<Mix />
	</button>
	<button class="active" onclick={() => navigate("/profile")}>
		<Person />
	</button>
	<button onclick={() => logout()}>
		<Exit />
	</button>
</nav>
