<script lang="ts">
	import { CrossCircled } from "svelte-radix";

	let username: string;
	let pass: string;
	let errorMessage: string = "";

	async function login() {
		let result = await auth();

		if (result?.user) {
			localStorage.setItem("userData", JSON.stringify(result.user));

			window.location.hash = "/profile";
		}
	}

	async function register() {
		try {
			let res = await fetch("http://127.0.0.1:8080/api/register", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ username, pass }),
			});

			if (!res.ok) {
				errorMessage = "Username already exists";
			}

			login();
		} catch (error) {
			errorMessage = "Login failed. Please try again.";
		}
	}

	async function auth() {
		try {
			let res = await fetch("http://127.0.0.1:8080/api/login", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ username, pass }),
			});

			if (!res.ok) {
				errorMessage = "Invalid login credentials";
				return null;
			}

			const userData = await res.json();
			return { user: userData };
		} catch (error) {
			errorMessage = "Login failed. Please try again.";
			console.error(error);
			return null;
		}
	}
</script>

{#if errorMessage}
	<div class="toast toast-top toast-center">
		<div class="alert alert-error flex items-center">
			<CrossCircled class="w-6 h-6 text-white" />
			<span class="text-white font-semibold">{errorMessage}</span>
		</div>
	</div>
{/if}

<div class="card glass w-80">
	<div class="card-body items-center text-center">
		<h2 class="card-title">Welcome!</h2>
		<p>Enter your details below to sign-in</p>
		<label class="input input-bordered flex items-center gap-2 w-full">
			Username
			<input type="text" class="grow" placeholder="Mr. Poopy Butthole" bind:value={username} />
		</label>
		<label class="input input-bordered flex items-center gap-2 w-full">
			Pin
			<input type="password" class="grow" placeholder="12345" bind:value={pass} />
		</label>
		<div class="card-actions mt-2">
			<button class="btn btn-primary" onclick={() => register()}>Register</button>
			<button class="btn btn-secondary" onclick={() => login()}>Login</button>
		</div>
	</div>
</div>
