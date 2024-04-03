# QR code generator 

Just a small projects, where I can try out [dioxus](https://dioxuslabs.com)

# What is it about

Just a small WebAssemlby compiled dioxus project. It generates a QR code with the [qrcode crate](https://docs.rs/qrcode/latest/qrcode/).

# How to build

This project uses [tailwindcss](https://tailwindcss.com/) and [daisyui](https://daisyui.com/).

To install those dependencies:

```sh
npm install
```

Then this projects also needs the dioxus-cli. You can install it with

```sh
cargo install dioxus-cli
```

as well as the webassembly target to compile. You can add this one with

```sh
rustup target add wasm32-unknown-unknown
```

If you installed all those dependencies, you can just start the
app with 

# How to run

```sh
./run.sh
```

This should serve the app at `localhost:8080`, which you can open with the browser of your choice!


Have fun
