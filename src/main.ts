import init, {sum_numbers, resize_image, open_image, resize_img_browser, resize_image_browser} from '../pkg/wasm_image_resizer.js';
// @is-ignore
import path from '../pkg/wasm_image_resizer_bg.wasm';
import App from './App.svelte';

const app = (async () => {
  await init(path);
  console.log('init');
  const app = new App({
    target: document.body,
    props: {
      name: 'Resize Image Benchmark',
      wasm: {sum_numbers, resize_image, open_image, resize_img_browser, resize_image_browser}
    },
  });
})();
export default app;
