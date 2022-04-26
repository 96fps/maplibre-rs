# Web

This document describes issues and challenges when packaging maplibre-rs as a npm package.

## Required Formats

### ESM

The ESM module format is the standard nowadays which should be followed. If a bundler like webpack encounters an ESM
module it can resolve WebAssembly files or WebWorkers dynamically.
The following syntax is used to resolve referenced WebWorkers:

```ts
new Worker(new URL("./pool.worker.ts", import.meta.url), {
    type: 'module'
});
```

Similarly, the following works:

```ts
new URL('index_bg.wasm', import.meta.url);
```

### IIFE (immediately-invoked function expression)

> This format is used when including maplibre-rs in a `<script>` tag. The library is "written" onto the window/global
> object. This allows quick prototyping/playgrounds/experiments using maplibre-rs.

In order to support this we need to create a bundle which works on any modern browser. Additionally, a WASM file and
WebWorker needs to be deployed at a predictable path, because there is no bundler active which manages assets.

Both assets could be inlined theoretically. This is common for WebWorkers, but not for WASM files.

### UMD

> UMD modules are needed when creating a library which should run in Node as well as browsers. This is not a usecase
> for maplibre-rs. If we support node, then we probably would ship a separate package called "maplibre-rs-node" which
> bundles to CJS directly.

### CJS/CommonJS

> Not needed for the browser build of maplibre-rs, possibly needed when supporting Node

With a CommonJS module its is not possible for bundlers to dynamically resolve WebWorkers or WASM files. Users of these
libraries have
to specify where WASM or non-inlined WebWorkers are hosted.

The `import.meta.url` token can not exist in a CommonJS module. Therefore, bundlers which encounter a CommonJS module
have to use a different mechanism of resolving files.

* The Parcel bundler translates to `new URL('index_bg.wasm', import.meta.url);`
  to `new URL("index_bg.wasm", "file:" + __filename);`
  While depending on `file:` and `filename` works for NodeJS, it is unsupported in the browser
* Webpack translates `new URL('index_bg.wasm', import.meta.url);` to something that is equivalent to `'./index_bg.wasm'`
  . It just expects that assets are resolvable from the current file.

## wasm-pack output

wasm-pack can output [multiple formats](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target). The `web`
and `bundler` outputs offer the most modular modules.
Unfortunately, the
function [wasm_bindgen::module()](https://docs.rs/wasm-bindgen/0.2.80/src/wasm_bindgen/lib.rs.html#1208-1217)
is only supported in `web` and `no-modules`. We currently are using this in order to send loaded instances
of `WebAssembly.Module` to WebWorkers. `nodejs` should not be used because MapLibre does not target Node.
Therefore, we should stick to the `web` output format.

## Required Features

* WASM Bundling
  > Make the WASM binary available to users of the maplibre-rs library
* WebWorker Bundling
  > Make the WebWorker available to users of the maplibre-rs library. This could also be achived by inlining.
* WebWorker Inlining
  > Inline the WebWorker bundle in the library bundle as a string.
* Predictable paths for CJS
  > TODO

## Bundler Feature Comparison

| Bundler       | **ESM** | **IIFE** | CJS | UMD | **WebWorker Inlining** | Web Worker Bundling | **WASM Bundling** | **
Predictable Paths** |
|---------------|---------|----------|-----|-----|------------------------|---------------------|-------------------|-----------------------|
| Babel 1)      | ✅       | ❌        | ❌   | ❌   | ❌                      | ❌                   | ❌                 | -                     |
| TypeScript 1) | ✅       | ❌        | ❌   | ❌   | ❌                      | ❌                   | ❌                 | -                     |
| Webpack       | ❌ 4)    | ❓        | ❌   | ❓   | ❌ 2)                   | ✅                   | ✅                 | ❓                     |
| Parcel        | ✅       | ❌        | ✅   | ❌   | 🛠️ 3)                 | ✅                   | ✅                 | ❌ 5)                  |
| ESBuild       | ✅       | ✅        | ✅   | ❌   | ✅ 6)                  | ❓                   | ✅ 6)              | ✅                     |
| Rollup        | ❓       | ❓        | ❓   | ❓   | ❓                      | ❓                   | ❓                 | ❓                     |

Features in **bold** are required for maplibre-rs.

> 1) Technically not a bundler but can be used to emit ES modules
> 2) Was Supported in Webpack 4, but currently is not supported
> 3) https://github.com/parcel-bundler/parcel/issues/8004
> 4) As of the time of writing Webpack can not output ESM libraries
> 5) Plugins exist, but they don't work reliably
> 6) Plugins exist, and work reliably

### Babel & TypeScript

Babel and TypeScript both can produce ESM modules, but they **fail with transforming references within the source code**
like `new URL("./pool.worker.ts", import.meta.url)`. There exist some Babel plugins, but none of them is stable.
Therefore, we actually need a proper bundler which supports outputting ESM modules.
The only stable solution to this is Parcel. Parcel also has good documentation around the bundling of WebWorkers.


### WebPack

TODO

### Parcel

TODO

### ESBuild

TODO

### Rollup

Not yet evaluated