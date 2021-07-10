# Yew Parcel Template

<div align="center">
    <p>
        <a href="https://discord.gg/ZHWmUaf" target="_blank"><img alt="Discord Chat" src="https://img.shields.io/badge/Discord-Spielrs%20-yellowgreen"/></a>
        <a href="https://github.com/spielrs/yew-parcel-template/blob/master/LICENSE" target="_blank"><img alt="License" src="https://img.shields.io/badge/License-MIT%20-lightgrey"></a>
        <a href="https://paypal.me/dancespiele?locale.x=en_US" target="_blank"><img alt="Donate by Paypal" src="https://img.shields.io/badge/Donate-PayPal-green.svg"/></a>
    </p>
</div>

**Kickstart your Yew, Yew-Router, WebAssembly, and Parcel project!**

This template comes pre-configured with all the boilerplate for compiling [Yew framework](https://yew.rs/docs/en/intro/) and [Yew Router](https://yew.rs/docs/en/concepts/router/)
to WebAssembly and hooking into a [Parcel build pipeline](https://parceljs.org/).

* `npm start` -- Serve the project locally for
  development at `http://localhost:1234` by default.

* `npm run build` -- Serve the project in production mode (gzip compression and high level wasm optimization)

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

You maybe want that parcel watch more than everything call by `index.html`, and the `src` and `Cargo.toml` of the crate, you need to add the path in the watcher of `wasm-builder`:

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

## Limitations
If the server is closed while pages are still serving and after it is running again, the browser will not synchronize again with the server until
you refresh the pages manually.

## Do you like Yew Parcel Template?
If you like Yew Parcel Template, help us supporting the project:
- BAT rewards in case that you use [Brave Browser](https://brave.com/)
- Using this link to create an account in [Binance](https://www.binance.com/en/register?ref=DB8EPXF0) (get 10% fee back for every trading)

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