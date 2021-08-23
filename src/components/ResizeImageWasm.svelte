<script>
	import { onMount, createEventDispatcher } from "svelte"

	export let objectURL
	export let originaltURL
	export let img
	export let canvas
	export let ctx

	const dispatch = createEventDispatcher();

	onMount(() => {
		console.log("ResizeImageWasm.svelte", "red", e)
		dispatch('message', {
			text: 'Hello!',
			objectURL: originaltURL
		});
	})


</script>

<strong>Red thing</strong>

<style>
	strong {
		color: red;
	}
</style>