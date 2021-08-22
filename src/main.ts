import init, {sum_numbers} from '../pkg/wasm_image_resizer.js';
// @is-ignore
import path from '../pkg/wasm_image_resizer_bg.wasm';
import App from './App.svelte';

const app = (async () => {
	await init(path);
	console.log("init")
	const app = new App({
		target: document.body,
		props: {
			name: 'Resize Image Benchmark',
			sum_numbers,
		},
	});
})();
export default app;
