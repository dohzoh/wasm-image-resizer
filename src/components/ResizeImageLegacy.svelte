<script>
	import { onMount, createEventDispatcher } from "svelte"

	export let objectURL
	export let originaltURL
	export let img
	export let canvas
	export let ctx

	const dispatch = createEventDispatcher();

	onMount(() => {
		console.log("ResizeImageLegacy.svelte.svelte", "green", e)
		dispatch('message', {
			text: 'Hello!',
			objectURL: originaltURL
		});
	})


</script>
	
<strong>Green thing</strong>

<style>
	strong {
		color: green;
	}
</style>