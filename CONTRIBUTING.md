# Contributing

- [Get Started](#get-started)
  - [Requirements](#requirements)
  - [First Setup](#first-setup)
    - [1. Git (with submodule)](#1-git-with-submodule)
    - [2. Core with Tauri \& tauri-driver](#2-core-with-tauri--tauri-driver)
    - [3. Webview with Yarn v3](#3-webview-with-yarn-v3)
    - [4. Check](#4-check)
  - [Local development](#local-development)
  - [Build a release](#build-a-release)
    - [Standalone Binary](#standalone-binary)
- [Style Guide \& Conventions](#style-guide--conventions)
  - [Tests](#tests)

## Get Started

### Requirements

- [Node.js](https://nodejs.org) or [nvm](https://github.com/nvm-sh/nvm#installing-and-updating): v18
- [Rust](https://www.rust-lang.org/tools/install): v1
- [Yarn](https://yarnpkg.com/getting-started/install): v1  
  _(we actually use Yarn v3 but it's bundled in all the latest Yarn v1 releases)_

### First Setup

> ⚠️ **Important**  
> If you're under **Windows**, you nust run all CLI commands under a Linux shell-like terminal (i.e.: WSL or Git Bash).

#### 1. Git (with submodule)

Then run:

```sh
git clone https://github.com/ivangabriele/clamav-desktop.git
cd ./clamav-desktop
git submodule init
git submodule update
```

#### 2. Core with Tauri & tauri-driver

To check the requirements related to Tauri and tauri-driver installations, please check
[this page](https://tauri.app/v1/guides/getting-started/prerequisites/#installing)
as well as [this one](https://tauri.app/v1/guides/testing/webdriver/ci/).

Once you're ready, you can run:

```sh
mkdir ./build
cd ./src-tauri
cp ./.cargo/config.toml.example ./.cargo/config.toml # and customize the content to match your local environment
cargo build
```

#### 3. Webview with Yarn v3

You may need to intall SDKs for your IDE/editor to handle Yarn v3: https://yarnpkg.com/getting-started/editor-sdks
(i.e.: `yarn dlx @yarnpkg/sdks vscode` if you're using VSCode).

Once you're ready, you can run:

```sh
cd .. # if you are still in `./src-tauri` directory
yarn
```

#### 4. Check

You should now be able to run `yarn dev` which will launch the application
(serving first the Webview on port 3000 and then launching the Core desktop app embedding this Webview).

### Local development

This will watch for file changes and automatically re-hydrate the webapp on the go:

```sh
yarn dev:docker
yarn dev
```

### Build a release

Keep in mind that building a release on your OS generally restrict the release generation to your OS (you can't natively
release a macOS `.dmg` under Ubuntu for example) but you can circumvent that by using VirtualBox (Docker is a hassle to embed macOS & Windows environments).

#### Standalone Binary

```sh
yarn release:bin
```

## Style Guide & Conventions

### Tests

For Rust unit tests, to avoid overwhelming main files with tests code, we follow 
[Google's C++ Style Guide](https://google.github.io/styleguide/cppguide.html#File_Names) and split unit test files
into a separate `filename_test.rs`.

The idea comes from this [Karol Kuczmarski's blog post](http://xion.io/post/code/rust-unit-test-placement.html).
