# Sukurappa

[![GitHub Actions workflow status](https://github.com/aeyoll/sukurappa/workflows/ci/badge.svg)](https://github.com/aeyoll/sukurappa/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.57.0+-lightgray.svg)](#rust-version-requirements)
[![Conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

Watch for changes on a webpage and do anything with it!

Install
---

With cargo:

```shell
cargo install sukurappa
```

Or use the install-script and add `$HOME/.sukurappa/bin` to your `$PATH`.

````shell
curl -fsSL https://raw.githubusercontent.com/aeyoll/sukurappa/main/install.sh | bash
````

Usage
---

```shell
# Add an url + selector to watch
sukurappa add --url https://example.org --selector "h1"

# Remove an url + selector to watch
sukurappa remove --url https://example.org --selector "h1"

# List watched urls + selectors
sukurappa list

# Watch for changes every 5 seconds and send an email with the new content every times it changes
sukurappa watch --command='mail -s "[Sukurappa] Content changed in URL" my@email.com <<< NEW_CONTENT' --frequency=5
```

Rust version requirements
---

Rust >= 1.57.0+