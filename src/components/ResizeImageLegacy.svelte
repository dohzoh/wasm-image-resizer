<script lang="ts">
	import { onMount, createEventDispatcher } from "svelte"

	export let objectURL: string
	export let originaltURL: string
	export let img: HTMLImageElement
	export let canvas: HTMLCanvasElement
	export let ctx: CanvasRenderingContext2D

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

		dispatch('message', {
			canvas,
			ctx,
		});
		console.log("ResizeImageLegacy end")
	})

	const toBlob = async(canvas: HTMLCanvasElement) => {
		return new Promise(function(resolve) {
			canvas.toBlob(resolve)
		})
	}
</script>

<style>
	strong {
		color: green;
	}
</style>