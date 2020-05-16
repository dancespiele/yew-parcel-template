# `yew-parcel-template`

**Kickstart your Yew, Yew-Router, WebAssembly, and Parcel project!**

This template comes pre-configured with all the boilerplate for compiling Yew framework and Yew Router
to WebAssembly and hooking into a Parcel build pipeline.

* `npm start` -- Serve the project locally for
  development at `http://localhost:1234` by default.

* `npm build` -- Serve the project in production mode


## Using This Template

```sh
cargo install wasm-pack
```

```sh
npm init yew-parcel my-app
```

```sh
npm install
```

## Configuration

You will want to change the name of your crate however also you need to change the import js file name that generate the wasm pack in the index.html
with the new name of your crate

```javascript
  <script type="module">
    import init, { run } from './name_of_your_crate.js';

    const start = async() => {
      await init('./name_of_your_crate_bg.wasm');
      run();
    };

    start();
  </script>
```

You maybe want that parcel watch more than evething call by `index.html`, and the `src` and `Cargo.toml` of the crate, you need to add the path in the 
watcher of `wasm-builder`:

```javascript
    chokidar.watch(['./crate/src', './crate/Cargo.toml', './crate/your_new_path_to_watch']).on('change', async (event, path) => {
        console.log(`there are new changes in '${path}'. Start to rebuild rustwasm sources`);

        bundler.bundle();

        bundler.hmr.broadcast({
            type: 'reload'
        });
    });
```

You can also change the address of the parcel server adding a .env file with the next setting:

```sh
SERVER_ADDRESS=[YOUR_ADDRESS]:[YOUR_PORT]
```

In case that you want to customize the wasm-pack build you will find it in `wasm_pack_cmd` sh. For powerShell you will need to write your own script
and change the name of execution file in `wasm-builder.js`:

```javascript
    bundler.on('buildStart', () => {
        const prevtBuildFile = Path.join(__dirname, './your_script.ps1');
        console.log(`running: ${prevtBuildFile}`);
        execSync(`${prevtBuildFile} ${buildType === 'production' ? '' : '--dev'}`, {stdio: 'inherit'});
    });

```

For static files you need to include in the `package.json`:

```json
"staticFiles": {
  "staticPath": ["./crate/pkg", "./crate/your_static_folder"],
  "watcherGlob": false
}
```

The static directory has to be included in some subfolder, not in the root path or it will be ignored. The bundle move the content of the directory
into the `dist` not the directory itself

## Why the template changes?
The pluging `parcel-pluging-wasm.rs` breaks since the version `0.2.61` of `wasm-bindgen` and it looks that is not maintained anymore.
Anyway although now small configurations is needed to add as I explained before, the new template build faster than the old one and also
the new build file `wasm-builder` is a good stuff for customization and extending the implementation.

## Limitations
If the server is closed while pages are still serving and after it is running again, the browser will not synchronize again with the server until
you refresh the pages manually.

## Rustc Version Required

- 1.40.0 or higher

## Attention

- if you don't want `web_sys`, change the feature of `yew` and `yew_router` to `std_web` in cargo.toml file and remove in your cargo config file:

```toml
[build]
target="wasm32-unknown-unknown"
```

### License

Yew Parcel Template is MIT licensed. See [license](LICENSE)