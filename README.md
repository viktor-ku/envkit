# envkit

> The stupid .env files management tool

Run `envkit --help` to get you started!

```
envkit --help
```

## Commands

### Diff

Shows missing variables in the `file_b` compared to the `file_a`. Unlike the standard `diff` program
it searches for the variable across the entire file.

## Get started

You have two options: install binary or build it yourself using `rustup`.

### Install

Run the following command in your terminal. It will download the latest available binary for your platform
from our CI/CD and install/upgrade it.

```bash
curl "https://gitlab.com/viktor-ku/envkit/raw/master/scripts/install.sh" | sh
```
