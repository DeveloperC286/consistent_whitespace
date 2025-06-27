# Consistent Whitespace
[![crates.io](https://img.shields.io/crates/v/consistent_whitespace)](https://crates.io/crates/consistent_whitespace)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

A utility that checks for consistent whitespace across your file(s).

## Installation
### From Cargo
```bash
cargo install consistent_whitespace
```

### From static binary
```bash
wget -O - "https://github.com/DeveloperC286/consistent_whitespace/releases/download/${consistent_whitespace_VERSION}/x86_64-unknown-linux-musl.gz" | gzip -d >/usr/bin/consistent_whitespace && chmod 755 /usr/bin/consistent_whitespace
```

### From Docker
```bash
docker pull ghcr.io/developerc286/consistent_whitespace:latest
```

## Usage
### Basic Usage
Check the current directory for consistent whitespace:
```bash
consistent_whitespace
```

Check specific files or directories:
```bash
consistent_whitespace path/to/file.txt path/to/directory/
```

### Whitespace Preferences
By default, the tool accepts either tabs or spaces as long as they are consistent within each file. You can enforce specific whitespace types:

```bash
consistent_whitespace --whitespace spaces
consistent_whitespace --whitespace tabs
consistent_whitespace --whitespace either
```

## GitHub Actions Example
```yaml
name: Continuous Integration (CI)

on: pull_request

permissions:
  contents: read

jobs:
  check-whitespace:
    name: Check Whitespace
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code.
        uses: actions/checkout@v4
      - name: Check whitespace consistency.
        uses: docker://ghcr.io/developerc286/consistent_whitespace:v0.6.0
        with:
          args: .
```

## Issues/Feature Requests
To report an issue or request a new feature use [https://github.com/DeveloperC286/consistent_whitespace/issues](https://github.com/DeveloperC286/consistent_whitespace/issues).