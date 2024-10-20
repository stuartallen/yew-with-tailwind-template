# A Template for Yew with Tailwind

This repository is a template for Yew setup with Tailwind. This is not an official template, I just thought it would be a nice starting point.

## What's provided

- An app that looks like [this](https://yew-app.fly.dev/)!
- Yew app with components and pages directory
- Prebuild hook for compiling tailwind styles in an ignored file
- Dockerfile for deploying on a service like fly.io

## Quick Start

- [Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Install cargo-generate](https://crates.io/crates/cargo-generate)
- [Install Trunk](https://trunkrs.dev/)
- [Install Tailwind Binaries](https://github.com/tailwindlabs/tailwindcss/releases/tag/v3.4.14) \*node is not necessary. Be sure to add this as tailwindcss to your path/profile so it can be used by the pre-build hook.
- I had to install wasm-unknown-unknown

```
rustup target add wasm32-unknown-unknown
```

### Or

serve this app just using Docker locally.

```
#   Build the image
#   Make take few minutes to compile the first time
docker build -t yew-tailwind-app .
#   Serve the app locally out of port 8080
docker run -d -p 8080:80 --name yew-tailwind-container yew-tailwind-app
```

## Deploying your application

Currently this application is set up for deployment with fly.io, but could be adjusted for other cloud providers or self-hosting.

- Install [fly CLI](https://fly.io/docs/flyctl/install/)

```
fly auth login
fly deploy
```

## Updates

None so far, this is the first available version!

## Issues?

Please let me know if you have issues, tips, and/or tricks! Heck, even if you just used/enjoyed this template. I am interested in maintaining this!

I can be reached through my portfolio site [here](https://www.stuartallensportfolio.com/) under "Contact & Socials". Thanks for using something I made!

## Special Thanks

- Thank you to all the developers of the [Yew](https://yew.rs/) framework!
- Thank you to Dmitry Slutskii for this guide on [setting up tailwind with Yew](https://lakret.net/blog/2023-03-10-tailwind-with-yew), this would be a fantastic resource for adding tailwind to an existing project. Just be sure to use this style tag in your project head:

```
<link data-trunk href="*path to compiled tailwind styles*" rel="css" />
```

- Thank you for checking out my template :)
