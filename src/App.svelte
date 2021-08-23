<script lang="ts">
	import ResizeImageWasm from './components/ResizeImageWasm.svelte';
	import ResizeImageLegacy from './components/ResizeImageLegacy.svelte';
	import ResizeImageCV from './components/ResizeImageCV.svelte';

	import {
		Col, Container, Row, 
		Image as CImage,
		Input,
		Button,
		Dropdown,
		DropdownItem,
		DropdownMenu,
		DropdownToggle,
		Modal,
		ModalBody,
		ModalFooter,
		ModalHeader
	} from 'sveltestrap'

	export let name: string;
	export let sum_numbers: Function;
	const sum = sum_numbers(new Int32Array([1, 20, 2000]))


	const childParams = {
		objectURL: <string> "",
		originaltURL: <string> "",
		img: <HTMLImageElement | null> null,
		canvas: <HTMLCanvasElement | null> null,
		ctx: <CanvasRenderingContext2D | null> null,
	}

	let imageFiles = [
		{name: "JohnDayRiver"},
		{name: "LakeMcDonald"},
		{name: "LoneStarGeyser"},
	]

	let solutions = [
		{name: "Legacy(Canvas)", method: "resizeImageLegacy", component: ResizeImageLegacy},
		{name: "WebAssembly", method: "resizeImageWasm", component: ResizeImageWasm},
		{name: "OpenCV", method: "resizeImageCV", component: ResizeImageCV},
	]
	let solutionName = "Choose One"
	let methodName = ""
	let fileName = ""
	let ignitionDisabled = "disabled"
	let currentComponent: Function

	const onChangeSelect = (e: Event) => {
		const index: number = parseInt((e.target as HTMLInputElement).value as string)
		solutionName = solutions[index].name
		methodName = solutions[index].method
		currentComponent = solutions[index].component
		checkIgnition();
	}

	const onChangeEvents = (e: Event) => {
		fileName = (e.target as HTMLInputElement).value as string
		checkIgnition();
	}

	const checkIgnition = () => {
		ignitionDisabled = (methodName !== "" && fileName !== "") ? "" : "disabled"
		console.log(ignitionDisabled)
	}

	let modalOpen = false;
	const onToggle = async() => {
		if(!modalOpen) {
			await preFilter()
		}		
		modalOpen = !modalOpen
	}

	const preFilter: Function = async() => {
		console.log("start preFIlter")
		childParams.objectURL = ""
		let url: string = "./img/"+fileName+".html.jpg"
		let resp: any = await fetch(url)
		const blob: Blob = await resp.blob()
		childParams.originaltURL = URL.createObjectURL(blob)
		console.log("originaltURL", childParams.originaltURL)

		console.time("Image Ready")
		let img: HTMLImageElement = new Image()
		img.src = childParams.originaltURL
		await img.decode()
		console.timeEnd("Image Ready")

		console.time("Canvas Ready")
		let canvas: HTMLCanvasElement = document.createElement('canvas');
		canvas.width = img.naturalWidth
		canvas.height = img.naturalHeight
		console.timeEnd("Canvas Ready")
		
		console.time("CanvasContext Ready")
		let ctx: CanvasRenderingContext2D = canvas.getContext('2d') as CanvasRenderingContext2D
//		ctx.drawImage(img, 0, 0);
		console.timeEnd("CanvasContext Ready")

		childParams.img = img
		childParams.canvas = canvas
		childParams.ctx = ctx
		console.log("end preFIlter")
	}


	const postFilter: Function = async(e: Event) => {
		console.log("postFilter start")
		const canvas = e.detail.canvas
		const ctx = e.detail.cts

		console.time("generate blob image")
		const blob = await toBlob(canvas)
		const objectURL = URL.createObjectURL(blob)
		console.time("generate blob image")

		console.time("Load resized image")
		const img: HTMLImageElement | null = document.querySelector("#resized_image") as HTMLImageElement
		img.src = objectURL
		await img.decode()
		img.width = img.naturalWidth
		img.height = img.naturalHeight
		console.timeEnd("Load resized image")

		console.log("Load Result end")
	}

	const toBlob = (canvas: HTMLCanvasElement) => {
		return new Promise(function(resolve) {
			canvas.toBlob(resolve)
		})
	}


</script>

<Container>
  <Row>
    <Col>
      <main>
		<h1>{name} {sum}</h1>
		<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
      </main>
	</Col>
  </Row>
	<Row>
		<Col xs="6">
			<Dropdown>
				<DropdownToggle caret>{solutionName}</DropdownToggle>
				<DropdownMenu>
					{#each solutions as { name, method }, index}
						<DropdownItem on:click={onChangeSelect} value={index}>
							{name}
						</DropdownItem>
					{/each}
				</DropdownMenu>
			</Dropdown>
		</Col>
		<Col xs="6"><br /><br /></Col>
	</Row>
	<Row>
{#each imageFiles as imageFile}
	  <Col>
		  <label>
			  <input type=radio name="image_file"
					 on:change={onChangeEvents}
					 value={imageFile.name}
					 bind:group={fileName} />
			  /img/{imageFile.name}.html.jpg
			  <CImage thumbnail alt="{imageFile.name}" src="/img/{imageFile.name}.html.jpg?random=1" />
		  </label>
	  </Col>
{/each}
  </Row>
	<Row>
		<Col>
			<h2>Current: {solutionName} {fileName}</h2><br />
		</Col>
	</Row>

	<Row>
		<Col></Col>
		<Col>
			<div>
				<Button class="btn btn-danger" disabled={ignitionDisabled} block on:click={onToggle}>Ignition</Button>
			</div>
		</Col>
		<Col></Col>
	</Row>
</Container>

<Modal isOpen={modalOpen} toggle={onToggle} size="xl">
    <ModalHeader toggle={onToggle}>Modal title</ModalHeader>
    <ModalBody>
      Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
      tempor incididunt ut labore et dolore magna aliqua.<br />
	  <svelte:component this={currentComponent} 
	    {...childParams}
		on:message={postFilter}
	  />

	  <CImage id="resized_image" />
    </ModalBody>
    <ModalFooter>
      <Button color="primary" on:click={onToggle}>Do Something</Button>
      <Button color="secondary" on:click={onToggle}>Cancel</Button>
    </ModalFooter>
  </Modal>


<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>

<svelte:head>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.0/dist/css/bootstrap.min.css">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.4.1/font/bootstrap-icons.css">
</svelte:head>

