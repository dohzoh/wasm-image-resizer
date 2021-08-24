<script lang="ts">
	import { onMount, createEventDispatcher } from "svelte"

	export let img: HTMLImageElement
	export let canvas: HTMLCanvasElement
	export let ctx: CanvasRenderingContext2D
	export let toBlob: Function

	const dispatch = createEventDispatcher();

	onMount(async() => {
		console.log("ResizeImageLegacy start")

		console.time("Resize image")
		ctx.drawImage(
			img, 
			0, 0,
			img.naturalWidth,
			img.naturalHeight,
			0, 0,
			canvas.width,
			canvas.height
		)
		console.timeEnd("Resize image")

		console.time("generate blob image")
		const blob = await toBlob(canvas)
		const objectURL = URL.createObjectURL(blob)
		console.timeEnd("generate blob image")

		dispatch('message', {
			canvas,
			ctx,
			objectURL,
		});
		console.log("ResizeImageLegacy end")
	})
</script>
