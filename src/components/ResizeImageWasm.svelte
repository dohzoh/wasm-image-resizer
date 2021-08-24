<script lang="ts">
	import { onMount, createEventDispatcher } from "svelte"

	export let img: HTMLImageElement
	export let canvas: HTMLCanvasElement
	export let ctx: CanvasRenderingContext2D
	export let resize_image: Function

	const dispatch = createEventDispatcher();

	onMount(async() => {
		console.log(canvas, ctx)
		console.log("ResizeImageLegacy start")

		console.time("Prepare Canvas");
		canvas.width = img.naturalWidth
		canvas.height = img.naturalHeight
		ctx.drawImage(img, 0, 0)
		console.timeEnd("Prepare Canvas");

		console.time("Resize image")
		const result: Uint8Array = resize_image(canvas, ctx, 512, 512, "jpg")
		console.timeEnd("Resize image")

		console.time("Uint8Array to Blob");
		const blob = new Blob([result]);
		console.log(`Resized: ${blob.size} Bytes`);
		console.timeEnd("Uint8Array to Blob");

		console.time("generate blob image")
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
