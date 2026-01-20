<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';

	let config = $state({
		appearance: {
			theme: 'system' as 'light' | 'dark' | 'system'
		},
		directories: {
			install: '',
			data: ''
		},
		launcher: {
			mode: 'exec' as 'exec' | 'steam-run',
			args: '',
			wrapper: '',
			dgpu: false,
			gamemode: false
		}
	});


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
				<option value="light">Light</option>
				<option value="dark">Dark</option>
				<option value="system">System</option>
			</select>
		</div>

	</section>

	<section>
		<h2 class="text-xl font-bold mb-2">Directories</h2>
		<div class="grid items-center grid-cols-[max-content_1fr_max-content] gap-2">
			<label for="install-dir">tModLoader installation:</label>
			<input id="install-dir" class="input-basic" type="text" bind:value={config.directories.install} />
			<button class="button-basic" onclick={() => pickDirectory('install')}>Browse</button>

			<label for="data-dir">tModLoader data:</label>
			<input id="data-dir" class="input-basic" type="text" bind:value={config.directories.data} />
			<button class="button-basic" onclick={() => pickDirectory('data')}>Browse</button>
		</div>
	</section>

	<section>
		<h2 class="text-xl font-bold mb-2">Launcher</h2>
		<div class="grid items-center grid-cols-[max-content_1fr] gap-2">
			<label for="launch-mode">Launch mode:</label>
			<select id="launch-mode" class="input-basic" bind:value={config.launcher.mode}>
				<option value="exec">Launch executable</option>
				<option value="steam-run">steam://run/1281930/{config.launcher.args}</option>
			</select>

			{#if config.launcher.mode === 'steam-run'}
				<div class="col-span-2 bg-gray-200 p-2 rounded">
					Some options may not be available in Steam launch mode.
				</div>
			{/if}

			<label for="game-args">Game arguments:</label>
			<input id="game-args" class="input-basic" type="text" bind:value={config.launcher.args} />

			<label for="wrapper">Wrapper command:</label>
			<input id="wrapper" class="input-basic" type="text" bind:value={config.launcher.wrapper} disabled={config.launcher.mode === 'steam-run'} />

			<label for="use-dgpu" class="col-span-2 checkbox-basic" >
				<input id="use-dgpu" type="checkbox" disabled={config.launcher.mode === 'steam-run'} />
				<span>Use discrete GPU</span>
			</label>

			<label for="use-gamemode" class="col-span-2 checkbox-basic">
				<input id="use-gamemode" type="checkbox" disabled={config.launcher.mode === 'steam-run'} />
				<span>Use FeralInteractive GameMode</span>
			</label>
		</div>
	</section>
</main>

