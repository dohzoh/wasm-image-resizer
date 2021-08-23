<script>
	import { onMount, createEventDispatcher } from "svelte"

	export let objectURL
	export let originaltURL
	export let img
	export let canvas
	export let ctx

	const dispatch = createEventDispatcher();

	onMount(async() => {
		console.log("ResizeImageLegacy start")

		console.time("resized")
		ctx.drawImage(
			img, 
			0, 0,
			img.naturalWidth,
			img.naturalHeight,
			0, 0,
			canvas.width,
			canvas.height
		)
		console.timeEnd("resized")

		console.time("generate blob image")
		const blob = await toBlob(canvas)
		const objectURL = URL.createObjectURL(blob)
        console.log(`Resized: ${blob.size} Bytes`);
		console.time("generate blob image")

		dispatch('message', {
			canvas,
			ctx,
		});
		console.log("ResizeImageLegacy end")
	})

	const toBlob = async(canvas) => {
		return new Promise(function(resolve) {
			canvas.toBlob(resolve)
		})
	}



</script>
	
<strong>Green thing</strong>

<style>
	strong {
		color: green;
	}
</style>