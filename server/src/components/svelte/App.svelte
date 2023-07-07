<script>
	import LeftPanel from "./LeftPanel.svelte";
	import RightPanel from "./RightPanel.svelte";
</script>

<div>
	<LeftPanel />
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
