#!/bin/bash
cd sample
rm -rf dist/script
npm run wasm
npm run link
npm run build
