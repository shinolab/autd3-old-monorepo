<script>
	import { options } from "./UI/options.js";
	import { invoke } from "@tauri-apps/api";
	import { appWindow } from "@tauri-apps/api/window";
	import { TauriEvent } from "@tauri-apps/api/event";
	import { onMount } from "svelte";

	import LeftPanel from "./LeftPanel.svelte";
	import RightPanel from "./RightPanel.svelte";

	onMount(async () => {
		options.set(await invoke("load_settings", {}));
	});

	const handleUnload = async () => {
		let tmp = null;
		options.subscribe((v) => {
			tmp = v;
		})();
		if (tmp) {
			let args = {
				options: JSON.stringify(tmp),
			};
			console.log(args);
			await invoke("save_settings", args);
		}
	};

	appWindow.listen(TauriEvent.WINDOW_CLOSE_REQUESTED, async () => {
		await handleUnload();
		await appWindow.close();
	});
</script>

<div>
	{#if $options}
		<LeftPanel options={$options} />
	{/if}
	<RightPanel />
</div>

<style>
	div {
		display: flex;
		width: 100%;
		align-items: flex-start;
		gap: 10px;
		flex-shrink: 0;

		padding: 10px;

		height: 100vh;
		box-sizing: border-box;
	}
</style>
