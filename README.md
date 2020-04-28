# `yew-parcel-template`

**Kickstart your Yew, Yew-Router, WebAssembly, and Parcel project!**

This template comes pre-configured with all the boilerplate for compiling Yew framework and Yew Router
to WebAssembly and hooking into a Parcel build pipeline.

* `npm run start` -- Serve the project locally for
  development at `http://localhost:1234`.

* `npm run build` -- Bundle the project (in production mode)


## Using This Template

```sh
cargo install wasm-pack
```

```sh
npm init yew-parcel my-app
```

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