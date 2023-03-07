# Contributing

- [Get Started](#get-started)
  - [Requirements](#requirements)
  - [First Setup](#first-setup)
  - [Local development](#local-development)
  - [Build a release](#build-a-release)
- [Style Guide \& Conventions](#style-guide--conventions)
  - [Tests](#tests)

## Get Started

### Requirements

- [Node.js](https://nodejs.org) or [nvm](https://github.com/nvm-sh/nvm#installing-and-updating): v18
- [Rust](https://www.rust-lang.org/tools/install): v1
- [Yarn](https://yarnpkg.com/getting-started/install): v1

### First Setup

> ⚠️ **Important**  
> If you're under **Windows**, you nust run all CLI commands under a Linux shell-like terminal (i.e.: Git Bash).

Then run:

```sh
git clone https://github.com/ivangabriele/clamav-desktop.git
cd ./clamav-desktop
git submodule init
git submodule update
yarn
```

For the Rust part please check [this page](https://tauri.app/v1/guides/getting-started/prerequisites/#installing)
as well as [this one](https://tauri.app/v1/guides/testing/webdriver/ci/) for requirements.

Once you're ready, you can run:

```sh
mkdir ./build
cd ./src-tauri
cargo build
cd ..
yarn dev
```

If you have the production `API_KEY` secret env, you can seed the production data locally by copy/pasting its value into
the `.env` file `PROD_API_SECRET` key and running `yarn dev:seed`.

> 📋 **Note**  
> The `yarn` command install the dependencies but also run the `scripts/dev/setup.js` scripts. This script does the
> following tasks, if necessary:
>
> - Copy `.env.example` file to a `.env` one.
> - Generate a RSA Key Pair (required in order to generate and verify [JWTs](https://jwt.io))

### Local development

This will watch for file changes and automatically re-hydrate the webapp on the go:

```sh
yarn dev:docker
yarn dev
```

### Build a release

```sh
yarn build
```

## Style Guide & Conventions

### Tests

For Rust unit tests, to avoid overwhelming main files with tests code, we follow 
[Google's C++ Style Guide](https://google.github.io/styleguide/cppguide.html#File_Names) and split unit test files
into a separate `filename_test.rs`.

The idea comes from this [Karol Kuczmarski's blog post](http://xion.io/post/code/rust-unit-test-placement.html).
