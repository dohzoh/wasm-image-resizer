<script lang="ts">
	import ResizeImageLegacy from './components/ResizeImageLegacy.svelte';
	import ResizeImageWasm from './components/ResizeImageWasm.svelte';
	import ResizeImageCV from './components/ResizeImageCV.svelte';

	import {
		Col, Container, Row, Image,
		Input,
		Button,
		Dropdown,
		DropdownItem,
		DropdownMenu,
		DropdownToggle,
		Modal,
		ModalBody,
		ModalFooter,
		ModalHeader,
	} from 'sveltestrap'

	export let name: string;
	export let sum_numbers: Function;
	const sum = sum_numbers(new Int32Array([1, 20, 2000]))

	let props = {}

	let imageFiles = [
		{name: "JohnDayRiver"},
		{name: "LakeMcDonald"},
		{name: "LoneStarGeyser"},
	]

	let solutions = [
		{name: "Legacy(Canvas)", method: "ResizeImageLegacy"},
		{name: "WebAssembly", method: "ResizeImageWasm"},
		{name: "OpenCV", method: "ResizeImageCV"},
	]
	let solutionName = "Choose One"
	let componentName = "ResizeImageLegacy"
	let methodName = ""
	let fileName = ""
	let ignitionDisabled = "disabled"
	let modalOpen = false

	const onChangeSelect = (e: Event) => {
		methodName = (e.target as HTMLInputElement).value as string
		componentName = methodName
		solutionName = (e.target as HTMLInputElement).textContent as string
		console.log("onChangeSelect", methodName)
		ignitionCheck()
	}

	const onChangeImage = (e: Event) => {
		fileName = (e.target as HTMLInputElement).value as string
		console.log("onChangeImage", fileName)
		ignitionCheck()
	}

	const ignitionCheck = () => {
		ignitionDisabled = (methodName !== "" && fileName !== "") ? "" : "disabled"
	}

	const onClickToggle = (e: Event) => {
		console.log("component", methodName)
		modalOpen = !modalOpen;
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
					{#each solutions as solution}
						<DropdownItem on:click={onChangeSelect} value={solution.method}>
							{solution.name}
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
					 on:change={onChangeImage}
					 value={imageFile.name}
					 bind:group={fileName} />
			  /img/{imageFile.name}.html.jpg
			  <Image thumbnail alt="{imageFile.name}" src="/img/{imageFile.name}.html.jpg?random=1" />
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
				<Button class="btn btn-danger" disabled={ignitionDisabled} block on:click={onClickToggle}>Ignition</Button>
			</div>
		</Col>
		<Col></Col>
	</Row>
</Container>

<Modal isOpen={modalOpen} toggle={onClickToggle} size="xl">
	<ModalHeader toggle={onClickToggle}>Modal title</ModalHeader>
	<ModalBody>
		Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
		tempor incididunt ut labore et dolore magna aliqua.

		{#if componentName === "ResizeImageLegacy"}
		<ResizeImageLegacy />
		{:else if  componentName === "ResizeImageWasm"}
		<ResizeImageWasm />
		{:else}
		<ResizeImageCV />	
		{/if}
		
	</ModalBody>
	<ModalFooter>
		<Button color="primary" on:click={onClickToggle}>Do Something</Button>
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

