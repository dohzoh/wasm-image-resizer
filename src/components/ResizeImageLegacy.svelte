<script>
import { onMount } from 'svelte';

	import {
		Image as CImage,
	} from 'sveltestrap'

    let objectURL
    let originaltURL
	let img
	let canvas
	let ctx

	const postFilter = (blob) => {
		objectURL = URL.createObjectURL(blob)
	}

    onMount(async () => {
        console.log("ResizeImageLegacy", "onMount", originaltURL)
        objectURL = originaltURL
    })

</script>

<h2>src/components/ResizeImageLegacy.svelte</h2>

{#if objectURL !== ""}
    <CImage alt="Landscape" src={objectURL} id="img_result" />
{/if}		
