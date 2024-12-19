<script lang="ts">
	import { onMount } from "svelte";

	let ws: any;

	let message = $state({
		text: "",
	});

	let input = $state({
		text: "",
	});

	const connect = () => {
		ws = new WebSocket("ws://127.0.0.1:8080/ws");
		ws.addEventListener("message", handleMsg);
		console.log("setup");

		return () => {
			ws.close();
			console.log("closed");
		};
	};

	function handleMsg(msg: any) {
		const data = JSON.parse(msg.data);
		console.log(data.text);
		message.text = data.text;
	}

	function sendMsg() {
		let msg = input;
		ws.send(JSON.stringify(msg));

		input.text = "";
		console.log("Message sent");
	}

	onMount(() => {
		connect();
	});
</script>

<h1 class="text-lg font-bold">{message.text}</h1>
<div class="join">
	<input bind:value={input.text} class="input input-bordered join-item" placeholder="Email" />
	<button onclick={() => sendMsg()} class="btn join-item rounded-r-full">Send</button>
</div>
