<p align="center">
  <img alt="ClamAV Desktop Logo" height="128" src="assets/icons/icon.svg" />
  <h1 align="center">ClamAV Desktop</h1>
</p>

<p align="center">
  <!--<a aria-label="Go to the latest Github release" href="https://github.com/ivangabriele/clamav-desktop/releases"><img alt="" aria-hidden="true" src="https://img.shields.io/github/v/release/ivangabriele/clamav-desktop?include_prereleases&sort=semver&style=for-the-badge&labelColor=000"></a>-->
  <a aria-label="Open the AGPL-3.0 license" href="https://github.com/ivangabriele/clamav-desktop/blob/main/LICENSE"><img alt="" aria-hidden="true" src="https://img.shields.io/github/license/ivangabriele/clamav-desktop?style=for-the-badge&labelColor=000"></a>
  <a aria-label="Go to the main branch unit workflow history" href="https://github.com/ivangabriele/clamav-desktop/actions?query=branch%3Amain+workflow%3AUnit"><img alt="" aria-hidden="true" src="https://img.shields.io/github/actions/workflow/status/ivangabriele/clamav-desktop/unit.yml?branch=main&label=Unit&style=for-the-badge&labelColor=000"></a>
  <a aria-label="Go to the main branch integration workflow history" href="https://github.com/ivangabriele/clamav-desktop/actions?query=branch%3Amain+workflow%3AIntegration"><img alt="" aria-hidden="true" src="https://img.shields.io/github/actions/workflow/status/ivangabriele/clamav-desktop/integration.yml?branch=main&label=E2E&style=for-the-badge&labelColor=000"></a>
  <a aria-label="Go to code coverage dashboard" href="https://app.codecov.io/github/ivangabriele/clamav-desktop"><img alt="" aria-hidden="true" src="https://img.shields.io/codecov/c/github/ivangabriele/clamav-desktop?style=for-the-badge&labelColor=000"></a>
  <a aria-label="Go to the ClamAV Desktop Stoat live chat server" href="https://stt.gg/xH2w834e" target="_blank" rel="noreferrer"><img alt="" aria-hidden="true" src="https://img.shields.io/badge/Live%20Chat-000?color=8C24EC&style=for-the-badge&labelColor=000&logo=revoltdotchat"></a>
</p>

> [!WARNING]  
> Clamav Desktop is a work in progress.

> [!NOTE]  
> This README is for _users_, rather than _contributors_. 
> If you wish to _contribute_ to ClamAV Desktop, you should read [CONTRIBUTING.md](CONTRIBUTING.md) instead.

---

- [Screenshots](#screenshots)
  - [Dashboard Preview](#dashboard-preview)
  - [Scanner Preview](#scanner-preview)
- [Roadmap](#roadmap)
    - [v.0.4.0](#v040)
    - [v.0.5.0](#v050)
    - [v.0.?.0](#v00)

---

## Screenshots

### Dashboard Preview

![ClamAV Desktop Dashboard](/docs/screenshot-dashboard.png)

### Scanner Preview

![ClamAV Desktop Scanner (file explorer)](/docs/screenshot-scanner-1.png)
![ClamAV Desktop Scanner (in progress)](/docs/screenshot-scanner-2.png)

## Roadmap

This roadmap is not set in stone and is prone to change unexpectedly while we're still in alpha (v0) stage.

#### v.0.4.0

- [x] Cloud: Definitions update
- [x] Dashboard: Daemon control & status
- [x] Global: Tray icon 
- [x] Scanner: Drives selection
- [x] Scanner: Folders selection
- [x] Scanner: Scan interruption
- [x] Settings: Raw clamd.conf editor 
- [ ] Technical: Releases in `.x64.deb`, `.x64.dmg`, and `.x64.msi` formats

#### v.0.5.0

- [ ] Scanner: Estimated time to completion
- [ ] Scanner: Quarantined files management
- [ ] Scanner: Summary
- [ ] Scanner: Threats handling management
- [ ] Technical: Store errors in local logs
- [ ] Technical: Releases in `.arm64.deb`, `.arm64.dmg`, and `.arm64.msi` formats

#### v.0.?.0

- [ ] Global: Automated updates (if/where possible)
- [ ] Publication: OpenSuse Build availibility (to challenge)
- [ ] Publication: PPA availibility
- [ ] Settings: Form-like `clamd.conf` editor
- [ ] Technical: macOS & Windows Code signing
- [ ] Technical: Binaries embedding (check with official ClamAV team)
- [ ] Technical: Releases `.x64.rpm` and `.arm64.rpm` formats  
  (Requires changes from tauri-apps/tauri#4402 & tauri-apps/tauri#5202 to be implemented.)
- [ ] UX/UI: Full redesign
