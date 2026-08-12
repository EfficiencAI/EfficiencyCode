<p align="center"><strong>EfficiencyCode CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/EfficiencyCode/blob/main/.github/EfficiencyCode-cli-splash.png" alt="EfficiencyCode CLI splash" width="80%" />
</p>
</br>
If you want EfficiencyCode in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/EfficiencyCode/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>EfficiencyCode app</code> or visit <a href="https://chatgpt.com/EfficiencyCode?app-landing-page=true">the EfficiencyCode App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>EfficiencyCode Web</strong>, go to <a href="https://chatgpt.com/EfficiencyCode">chatgpt.com/EfficiencyCode</a>.</p>

---

## Quickstart

### Installing and running EfficiencyCode CLI

Run the following on Mac or Linux to install EfficiencyCode CLI:

```shell
curl -fsSL https://chatgpt.com/EfficiencyCode/install.sh | sh
```

Run the following on Windows to install EfficiencyCode CLI:

```shell
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/EfficiencyCode/install.ps1 | iex"
```

The standalone installers download from `https://releases.openai.com/EfficiencyCode` by default and fall back to GitHub Releases if a metadata or asset download is unavailable. To force GitHub Releases, set `EfficiencyCode_INSTALLER_USE_RELEASES_OPENAI_COM` to `false` (`0` and `no` are also accepted):

```shell
curl -fsSL https://chatgpt.com/EfficiencyCode/install.sh | EfficiencyCode_INSTALLER_USE_RELEASES_OPENAI_COM=false sh
```

```powershell
$env:EfficiencyCode_INSTALLER_USE_RELEASES_OPENAI_COM='false'; irm https://chatgpt.com/EfficiencyCode/install.ps1 | iex
```

EfficiencyCode CLI can also be installed via the following package managers:

```shell
# Install using npm
npm install -g @openai/EfficiencyCode
```

```shell
# Install using Homebrew
brew install --cask EfficiencyCode
```

Then simply run `EfficiencyCode` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/EfficiencyCode/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `EfficiencyCode-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `EfficiencyCode-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `EfficiencyCode-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `EfficiencyCode-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `EfficiencyCode-x86_64-unknown-linux-musl`), so you likely want to rename it to `EfficiencyCode` after extracting it.

</details>

### Using EfficiencyCode with your ChatGPT plan

Run `EfficiencyCode` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use EfficiencyCode as part of your Plus, Pro, Business, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-EfficiencyCode-in-chatgpt).

You can also use EfficiencyCode with an API key, but this requires [additional setup](https://developers.openai.com/EfficiencyCode/auth#sign-in-with-an-api-key).

## Docs

- [**EfficiencyCode Documentation**](https://developers.openai.com/EfficiencyCode)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
