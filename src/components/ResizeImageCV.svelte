<script lang="ts">
	import { onMount, createEventDispatcher } from "svelte"

	export let img: HTMLImageElement
	export let canvas: HTMLCanvasElement
	export let ctx: CanvasRenderingContext2D
	export let cv: any
	export let toBlob: Function

	const dispatch = createEventDispatcher();

	onMount(async() => {
		console.log("ResizeImageCV start")

		console.time("Resize image")

		console.time("create dense array");
		let src = cv.imread(img);
		let dst = new cv.Mat();
		console.timeEnd("create dense array");

		console.time("cv::resize()");
		let dsize = new cv.Size(300, 300);
		// You can try more different parameters
		cv.resize(src, dst, dsize, 0, 0, cv.INTER_AREA);
		console.timeEnd("cv::resize()");

		console.time("cv::imshow()");
		cv.imshow(canvas, dst);
		console.timeEnd("cv::imshow()");

		src.delete(); dst.delete();

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
		console.log("ResizeImageCV end")
	})
</script>
