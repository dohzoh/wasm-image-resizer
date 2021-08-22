import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
import sveltePreprocess from 'svelte-preprocess';
import css from 'rollup-plugin-css-only';
import {rawWasm} from '@wesley-clements/rollup-plugin-raw-wasm';
import esbuild from 'rollup-plugin-esbuild';
import serve from 'rollup-plugin-serve';

const production = !process.env.ROLLUP_WATCH;

export default {
  input: 'src/main.ts',
  output: {
    sourcemap: true,
    format: 'iife',
    name: 'app',
    file: 'public/build/bundle.js',
  },
  plugins: [
    svelte({
      preprocess: sveltePreprocess({sourceMap: !production}),
      compilerOptions: {
        // Enable run-time checks when not in production
        dev: !production,
      },
    }),
    // We'll extract any component CSS out into
    // a separate file - better for performance
    css({output: 'bundle.css'}),

    // If you have external dependencies installed from
    // npm, you'll most likely need these plugins. In
    // some cases you'll need additional configuration -
    // consult the documentation for details:
    // https://github.com/rollup/plugins/tree/master/packages/commonjs
    resolve({
      browser: true,
      dedupe: ['svelte'],
    }),
    commonjs(),
    esbuild({
      sourceMap: !production,
      minify: production,
    }),
    rawWasm({
      publicPath: '/build/',
    }),
    // In dev mode, call `npm run start` once
    // the bundle has been generated
    !production && serve({
      contentBase: 'public',
      mimeTypes: {
        'application/wasm': ['wasm'],
      },
    }),

    // Watch the `public` directory and refresh the
    // browser on changes when not in production
    !production && livereload('public'),

    // If we're building for production (npm run build
    // instead of npm run dev), minify
  ],
  watch: {
    clearScreen: false,
  },
};
