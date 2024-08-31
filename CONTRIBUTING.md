# Contributing

- [Personal Note](#personal-note)
- [Getting started](#getting-started)
  - [Requirements](#requirements)
  - [First setup](#first-setup)
    - [1. Git (with submodules)](#1-git-with-submodules)
    - [2. Webview with Yarn v3](#2-webview-with-yarn-v3)
    - [3. Core with Tauri \& tauri-driver](#3-core-with-tauri--tauri-driver)
    - [4. Final Check](#4-final-check)
    - [5. Optional requirements](#5-optional-requirements)
  - [Local development](#local-development)
- [Build a release](#build-a-release)
  - [Binary (standalone)](#binary-standalone)
  - [Debian-based OS (deb)](#debian-based-os-deb)
  - [macOS (dmg)](#macos-dmg)
  - [Windows 8+ (msi)](#windows-8-msi)
- [Tests](#tests)
  - [Core unit tests](#core-unit-tests)
  - [Webview unit tests](#webview-unit-tests)
  - [E2E tests](#e2e-tests)
- [Style Guide \& Conventions](#style-guide--conventions)
  - [Tests](#tests-1)
  - [Commit messages](#commit-messages)
    - [Conventional Commit Types](#conventional-commit-types)
    - [Conventional Commit Scopes](#conventional-commit-scopes)
- [IDEs Configuration](#ides-configuration)
  - [Visual Studio Code](#visual-studio-code)

## Personal Note

The code is kind of a mess right now but is cleaned little by little in preparation for the v0.4.0.

Even though I'm 20y+ experienced in development, I'm a beginner in Rust and in "low level" languages. Once the code will
start to stabilize (starting from v0.4.0 release), I'll be happy to welcome any experienced Rust developer who wished to
contribute with better patterns, dryer code and some optimizations.

I'll also be happy to welcome any contributor/ion in general! I know I won't be able to handle everything alone on my
free time. Just be prepared, I am nit-picky regarding the harmony of naming, structures and conventions (from code to
commit messages) despite the current state of the codebase. But I'm trying to take it easier!

Also, at some point in the future, I do hope to find a good UX/UI designer willing to contribute because I think
opensource can look fancy (yes it matters for non-power users!) while being accessible to the largest number possible.
And let's say ~~the~~ my truth, most opensource softwares are not really appealing in terms of both UX & UI (which
doesn't make these projects and their contributors any less amazing, it's just a personal take on this matter).

As a final note, you'll notice that you have to install ClamAV binaries separately before being able to use ClamAV
Desktop. I talked with one core ClamAV contributors (a year ago) and we may hopefully work together at some point, so
that they can help us build and embed the binaries to make end-users life easier with an almost-single-click
installation.

## Getting started

### Requirements

- [Node.js](https://nodejs.org) or [nvm](https://github.com/nvm-sh/nvm#installing-and-updating): v18
- [Yarn](https://yarnpkg.com/getting-started/install): v1  
  _(we actually use Yarn v4 but it's bundled in all the latest Yarn v1 releases)_
- [Rust](https://www.rust-lang.org/tools/install): v1
- [clamav](https://github.com/Cisco-Talos/clamav): binaries must be available as global commands (PATH)
- make: v4

### First setup

> [!IMPORTANT]  
> If you're under **Windows**, you nust run all CLI commands under a Linux shell-like terminal (i.e.: WSL or Git Bash).

#### 1. Git (with submodules)

Then run:

```sh
git clone https://github.com/ivangabriele/clamav-desktop.git # or your fork
cd ./clamav-desktop
git submodule init
git submodule update
```

#### 2. Webview with Yarn v3

You may need to intall SDKs for your IDE/editor to handle Yarn v3: https://yarnpkg.com/getting-started/editor-sdks
(i.e.: `yarn dlx @yarnpkg/sdks vscode` if you're using VSCode).

Once you're ready, you can run:

```sh
cd .. # if you are still in `./src-tauri` directory
yarn
```

#### 3. Core with Tauri & tauri-driver

To check the requirements related to Tauri, WebKitWebDriver (Linux) amd tauri-driver installations, please check:

- [Tauri Installation Guide](https://tauri.app/v1/guides/getting-started/prerequisites/#installing)
- [Tauri WebDriver System Dependencies](https://tauri.app/v1/guides/testing/webdriver/introduction/#system-dependencies)
- [Tauri WebDriver CI Guide](https://tauri.app/v1/guides/testing/webdriver/ci/)

Once you're ready, you can run:

```sh
cargo install tauri-driver
cd ./src-tauri
cp ./.cargo/config.toml.example ./.cargo/config.toml # and customize the content to match your local environment
cargo build
```

#### 4. Final Check

You should now be able to run `yarn dev` which will launch the application (serving first the Webview on port 1420 and
then launching the Core desktop app embedding this Webview).

#### 5. Optional requirements

- [cargo-deb](https://github.com/kornelski/cargo-deb#installation) for debian bundle packaging.
- [cargo-edit](https://github.com/killercup/cargo-edit). for `cargo upgrade`-related commands (i.e.: `make upgrade`)
- [cargo-watch](https://github.com/watchexec/cargo-watch#install) for `cargo watch`-related commands (i.e.:
  `make test-*-watch`).
- [ggshield](https://github.com/GitGuardian/ggshield#installation) for `yarn test:sec` command.

### Local development

This will watch for file changes and automatically re-hydrate the webapp on the go:

```sh
yarn dev
```

## Build a release

Keep in mind that building a release on your OS generally restrict the release generation to your OS (you can't natively
release a macOS `.dmg` under Ubuntu for example) but you can circumvent that by using VirtualBox (Docker is a hassle to
embed macOS & Windows environments).

### Binary (standalone)

```sh
yarn bundle:bin
```

### Debian-based OS (deb)

```sh
yarn bundle:deb
```

### macOS (dmg)

```sh
yarn bundle:dmg
```

### Windows 8+ (msi)

```sh
yarn bundle:msi:arch64
yarn bundle:msi:x32
yarn bundle:msi:x64
```

## Tests

### Core unit tests

```sh
yarn test:unit:core
```

### Webview unit tests

```sh
yarn test:unit:webview
```

### E2E tests

```sh
yarn bundle:bin
yarn test:e2e
```

## Style Guide & Conventions

### Tests

For Rust unit tests, to avoid overwhelming main files with tests code, we follow
[Google's C++ Style Guide](https://google.github.io/styleguide/cppguide.html#File_Names) and split unit test files into
a separate `filename_test.rs`.

The idea comes from this [Karol Kuczmarski's blog post](http://xion.io/post/code/rust-unit-test-placement.html).

### Commit messages

#### Conventional Commit Types

Commit messages must follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) with this types:

- `build`: Anything related to application run or build (build configs, dev configs, scripts, dependencies, etc).
- `ci`: Anything related to continuous integration, including locally (Github Actions, husky, Git, etc).
- `docs`: Anything related to documentation (readme, contributing, website).
- `feat`: Any feature addition, edition or removal.
- `fix`: Anything related to documentation (readme, contributing, website).
- `perf`: Any application performance-related change.
- `refactor`: Any change in the structure/codebase that doesn't add or change a feature.
- `style` Any application UI-releated change (components, styles, assets, etc).
- `test`: Any unit or integration tests change.

`feat`, `fix`, `perf`, `refactor`, `style` & `test` types only concerns fixes related to the application istself, as
defined by the scopes below.

For everything else (i.e.: a CI fix, a performance improvement for a script, etc), use either `build`, `ci` or `docs`,
depending on the context.

You have to keep in mind that `feat`, `fix`, `perf` and `style` will end up in the end-users changelog while other will
not. It's there to help them understand what changed since the last version when a new release is published.

#### Conventional Commit Scopes

Our official `feat`, `fix`, `perf`, `refactor`, `style` & `test` scopes are:

- `cloud`: Any change impacting the Cloud screen.
- `dashboard`: Any change impacting the Dashboard screen.
- `scanner`: Any change impacting the Scanner screen.
- `settings`: Any change impacting the Settings screen.

## IDEs Configuration

### Visual Studio Code

`.vscode/extensions.json`

```json
{
  "recommendations": [
    "biomejs.biome",
    "coolbear.systemd-unit-file",
    "fill-labs.dependi",
    "editorconfig.editorconfig",
    "esbenp.prettier-vscode",
    "idleberg.nsis",
    "rust-lang.rust-analyzer",
    "tamasfe.even-better-toml",
    "tauri-apps.tauri-vscode"
  ]
}
```

`.vscode/settings.json`

```json
{
  "editor.defaultFormatter": "biomejs.biome",
  "[javascript]": {
    "editor.defaultFormatter": "biomejs.biome"
  },
  "[javascriptreact]": {
    "editor.defaultFormatter": "biomejs.biome"
  },
  "[json]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[typescript]": {
    "editor.defaultFormatter": "biomejs.biome"
  },
  "[typescriptreact]": {
    "editor.defaultFormatter": "biomejs.biome"
  }
}
```
