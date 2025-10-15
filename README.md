# Model Manager for Generative AI Tools

## Usage

This will download `Pony Diffusion V6` from civitai to `./models/checkpoints/pony@6.0.0.safetensors` and generate a `models.yml` file to reinstall models.

```bash
# Generate this on the CIVITAI website and set it in your environment
export CIVITAI_TOKEN="token"

mpm add pony@6.0.0
```

### Without Version Specifiers

```bash
# Will check the latest version and download it
mpm add pony
```

### Install From a `models.yml` file

If you have a perviously generated `models.yml` file

```yaml
checkpoints:
  pony: 6.0.0
  hassaku: 1.3.0-a
loras:
  genesis: 1.0.0
```

Then you can just run

```bash
mpm install
```

# Install `mpm`

On Linux or MacOS download the latest binary in the [GitHub releases](https://github.com/alshdavid/mpm/releases/latest) then run it from your terminal.

## Auto-install Script

Alternatively, you can use the following install script on Linux, MacOS and Windows (via Git Bash). Re-running the script updates the version.

```bash
# Linux/MacOS - Installs to $HOME/.local/mpm
# Windows     - Installs to C:\Users\username\.local/mpm

# Manually add the install path to $PATH (.zshrc, .bashrc) or Windows Environment Manager

eval $(curl -sSf "sh.davidalsh.com/mpm.sh" | sh -s)
mpm --help
```


