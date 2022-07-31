# Contributing

- [Get Started](#get-started)
  - [Requirements](#requirements)
  - [First Setup](#first-setup)
  - [Local development](#local-development)
  - [Build a release](#build-a-release)
- [IDEs](#ides)
  - [Recommended Visual Studio Code settings](#recommended-visual-studio-code-settings)

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
cd clamav-desktop
git submodule init
yarn
cd src-tauri
cargo
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

## IDEs

### Recommended Visual Studio Code settings

`.vscode/settings.json`

```json
{
  "editor.codeActionsOnSave": {
    "source.fixAll": true
  },
  "editor.defaultFormatter": "dbaeumer.vscode-eslint",
  "editor.formatOnSave": true,
  "eslint.codeActionsOnSave.mode": "all",
  "eslint.format.enable": true,
  "eslint.packageManager": "yarn",
  "[css]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[json]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[prisma]": {
    "editor.defaultFormatter": "Prisma.prisma"
  }
}
```
