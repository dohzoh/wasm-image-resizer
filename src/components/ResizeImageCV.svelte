<script>
	import { onMount, createEventDispatcher } from "svelte"

	export let objectURL
	export let originaltURL
	export let img
	export let canvas
	export let ctx

	const dispatch = createEventDispatcher();

	onMount(() => {
		console.log("ResizeImageCV.svelte", "Blue", originaltURL)
		dispatch('message', {
			text: 'Hello!',
			objectURL: originaltURL
		});
	})


</script>

<strong>Blue thing</strong>

<style>
	strong {
		color: blue;
	}
</style>