# Consistent Whitespace
[![crates.io](https://img.shields.io/crates/v/consistent_whitespace)](https://crates.io/crates/consistent_whitespace)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

A utility that checks for consistent whitespace across your file(s).

- [Usage](#usage)
- [Examples](#examples)
  - [GitHub Actions](#github-actions)
  - [GitLab CI](#gitlab-ci)
- [Installation](#installation)
  - [Binary](#binary)
  - [Cargo](#cargo)
  - [Docker](#docker)

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

## Examples
### GitHub Actions
<!-- x-release-please-start-version -->
```yaml
name: Continuous Integration (CI)

on: pull_request

permissions:
  contents: read

jobs:
  check-whitespace:
    name: Check Whitespace
    runs-on: ubuntu-24.04
    steps:
      - name: Checkout code.
        uses: actions/checkout@v4
      - name: Check whitespace consistency.
        uses: docker://ghcr.io/developerc286/consistent_whitespace:0.7.2
        with:
          args: .
```
<!-- x-release-please-end -->

### GitLab CI
<!-- x-release-please-start-version -->
```yaml
check-whitespace:
  image: ghcr.io/developerc286/consistent_whitespace:0.7.2
  script:
    - consistent_whitespace .
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```
<!-- x-release-please-end -->

## Installation
### Binary
<!-- x-release-please-start-version -->
```sh
version="v0.7.2" && wget -O - "https://github.com/DeveloperC286/consistent_whitespace/releases/download/${version}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
```
<!-- x-release-please-end -->

### Cargo
<!-- x-release-please-start-version -->
```bash
cargo install consistent_whitespace@0.7.2
```
<!-- x-release-please-end -->

### Docker
You can use the Docker image published to [ghcr.io/developerc286/consistent_whitespace](https://github.com/DeveloperC286/consistent_whitespace/pkgs/container/consistent_whitespace).

<!-- x-release-please-start-version -->
```bash
docker run --rm -v $(pwd):/workspace -w /workspace ghcr.io/developerc286/consistent_whitespace:0.7.2 .
```
<!-- x-release-please-end -->

### Issues/Feature Requests
Report issues or request features on our [GitHub Issues](https://github.com/DeveloperC286/consistent_whitespace/issues).
