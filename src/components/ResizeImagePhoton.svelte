<script lang="ts">
	import { onMount, createEventDispatcher } from "svelte"

	export let img: HTMLImageElement
	export let canvas: HTMLCanvasElement
	export let ctx: CanvasRenderingContext2D
	export let toBlob: Function
	export let wasm: {
		open_image: Function,
		resize_img_browser: Function,
		resize_image_browser: Function,
	};

	const dispatch = createEventDispatcher();

	onMount(async() => {
		console.log("ResizeImageLegacy start")

		console.time("Prepare Canvas");
		canvas.width = img.naturalWidth
		canvas.height = img.naturalHeight
		ctx.drawImage(img, 0, 0)
		console.timeEnd("Prepare Canvas");

		console.time("Resize image")
		const result: HTMLCanvasElement = wasm.resize_image_browser(canvas, ctx, 512, 512, 1)
		console.timeEnd("Resize image")

		console.time("generate blob image")
		const blob = await toBlob(result)
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
