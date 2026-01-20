<script lang="ts">
	import '../app.css'
	import '../bindings'
	import { invoke } from "@tauri-apps/api/core";
	import { type Config } from '../bindings';

	(async () => {
		const config = await invoke<Config>('load_config');
		console.log('Loaded config:', config);
	})();
</script>

<slot />