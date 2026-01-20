<script lang="ts">
	import { event } from "@tauri-apps/api";
	import { invoke } from "@tauri-apps/api/core";

	let tmodDir: string;

	async function save(e: SubmitEvent) {
		e.preventDefault();
	}

	async function pickTmodDir() {
		console.log("Picking tModLoader directory...");
		await invoke("pick_tmod_dir");
	}
</script>

<main class="container p-4">
	<h1>tModLoader Instances</h1>

	<form class="flex flex-col" onsubmit={save}>
		<button class="mb-4 p-2 border rounded cursor-pointer" onclick={pickTmodDir}>Pick tModLoader directory</button>
		<!-- <p>{tmodDir}</p> -->

		<button type="submit">Save</button>
	</form>
</main>

