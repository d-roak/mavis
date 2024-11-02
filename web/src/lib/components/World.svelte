<script>
	import { onMount } from 'svelte';
	import __wbg_init from '../../assets/bevy/mavis-world';

	onMount(async () => {
		await __wbg_init();
	});
</script>

<canvas id="game_canvas" />
