<script lang="ts">
	import {
		Col, Container, Row, Image,
		Input,
		Button,
		Dropdown,
		DropdownItem,
		DropdownMenu,
		DropdownToggle
	} from 'sveltestrap'

	export let name: string;
	export let sum_numbers: Function;
	const sum = sum_numbers(new Int32Array([1, 20, 2000]))

	let imageFiles = [
		{name: "JohnDayRiver"},
		{name: "LakeMcDonald"},
		{name: "LoneStarGeyser"},
	]

	let solutions = [
		{name: "Legacy(Canvas)", method: "resizeImageLegacy"},
		{name: "WebAssembly", method: "resizeImageWasm"},
		{name: "OpenCV", method: "resizeImageCV"},
	]
	let solutionName = "Choose One"
	let methodName = ""
	let fileName = ""
	let ignitionDisabled = "disabled"

	const onChangeSelect = (e: Event) => {
		methodName = (e.target as HTMLInputElement).value as string
		solutionName = (e.target as HTMLInputElement).textContent as string
		onChangeEvents(e)
	}

	const onChangeEvents = (e: Event) => {
		ignitionDisabled = (methodName !== "" && fileName !== "") ? "" : "disabled"
		console.log(ignitionDisabled)
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
					 on:change={onChangeEvents}
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
				<Button class="btn btn-danger" disabled={ignitionDisabled} block>Ignition</Button>
			</div>
		</Col>
		<Col></Col>
	</Row>
</Container>

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

