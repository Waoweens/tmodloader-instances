<script lang="ts">
	import { beforeNavigate } from "$app/navigation";
	import { commands } from "$lib/bindings";
	import { get } from "svelte/store";
	import type { PageProps } from "./$types";
	import { open } from "@tauri-apps/plugin-dialog";
	
	let { data }: PageProps = $props();

	// svelte-ignore state_referenced_locally
	let config = $state($state.snapshot(data.config));

	let gameArgumentsString = $state('');
	let wrapperCommandsString = $state('');

	let gameArgumentsArray = $derived(
		gameArgumentsString.split(' ')
	);

	let wrapperCommandsArray = $derived(
		wrapperCommandsString.split(' ')
	);

	// Initialize strings from config ONCE
	$effect(() => {
		gameArgumentsString = config.launcher.game_arguments.join(' ');
		wrapperCommandsString = config.launcher.wrapper_commands.join(' ');
	});

	// Sync strings → config
	$effect(() => {
		config.launcher.game_arguments = gameArgumentsArray;
		config.launcher.wrapper_commands = wrapperCommandsArray;
	});

	// Persist config
	$effect(() => {
		const snapshot = $state.snapshot(config);
		console.log('Config changed:', snapshot);

		commands.setConfig(snapshot);
		commands.saveConfig();
	});

	beforeNavigate(() => {
		commands.saveConfig();
	})

	async function pickDirectory(key: keyof typeof config.directories) {
		const result = await open({
			directory: true,
			multiple: false,
		});

		if (typeof result === 'string') {
			config.directories[key] = result;
		}
	}

</script>

<main class="p-4 flex flex-col gap-2">
	<a href="/">← Back</a>
	<h1 class="text-2xl font-bold mb-2">Settings</h1>

	<section>
		<h2 class="text-xl font-bold mb-2">Appearance</h2>
		<div class="grid items-center grid-cols-[max-content_max-content] gap-2">
			<label for="theme-select">Theme:</label>
			<select id="theme-select" class="input-basic" bind:value={config.appearance.theme}>
				<option value="Light">Light</option>
				<option value="Dark">Dark</option>
				<option value="System">System</option>
			</select>
		</div>

	</section>

	<section>
		<h2 class="text-xl font-bold mb-2">Directories</h2>
		<div class="grid items-center grid-cols-[max-content_1fr_max-content] gap-2">
			<label for="install-dir">tModLoader installation:</label>
			<input id="install-dir" class="input-basic" type="text" bind:value={config.directories.tmodloader_installation} />
			<button class="button-basic" onclick={() => pickDirectory('tmodloader_installation')}>Browse</button>

			<label for="data-dir">tModLoader data:</label>
			<input id="data-dir" class="input-basic" type="text" bind:value={config.directories.tmodloader_data} />
			<button class="button-basic" onclick={() => pickDirectory('tmodloader_data')}>Browse</button>
		</div>
	</section>

	<section>
		<h2 class="text-xl font-bold mb-2">Launcher</h2>
		<div class="grid items-center grid-cols-[max-content_1fr] gap-2">
			<label for="launch-mode">Launch mode:</label>
			<select id="launch-mode" class="input-basic" bind:value={config.launcher.launch_mode}>
				<option value="Execute">Launch executable</option>
				<option value="SteamRun">steam://run/1281930/{config.launcher.game_arguments}</option>
			</select>

			{#if config.launcher.launch_mode === 'SteamRun'}
				<div class="col-span-2 bg-gray-200 p-2 rounded">
					Some options may not be available in Steam launch mode.
				</div>
			{/if}

			<label for="game-args">Game arguments:</label>
			<input id="game-args" class="input-basic" type="text" bind:value={gameArgumentsString} />

			<label for="wrapper">Wrapper command:</label>
			<input id="wrapper" class="input-basic" type="text" bind:value={wrapperCommandsString} disabled={config.launcher.launch_mode === 'SteamRun'} />

			<label for="use-dgpu" class="col-span-2 checkbox-basic" >
				<input id="use-dgpu" type="checkbox" bind:checked={config.launcher.use_dgpu} disabled={config.launcher.launch_mode === 'SteamRun'} />
				<span>Use discrete GPU</span>
			</label>

			<label for="use-gamemode" class="col-span-2 checkbox-basic">
				<input id="use-gamemode" type="checkbox" bind:checked={config.launcher.use_gamemode} disabled={config.launcher.launch_mode === 'SteamRun'} />
				<span>Use FeralInteractive GameMode</span>
			</label>
		</div>
	</section>
</main>

