# nclip

Clipboard share remote and local.

required: SSH remote port forwarding

replace nclip (https://github.com/pyar6329/nclip-legacy) by Rust


# Installation

```bash
$ curl -sL -o nclip.tar.zst https://github.com/pyar6329/nclip/releases/latest/download/nclip-$(uname -s)-$(uname -m).tar.zst
$ tar -I pzstd -xvf nclip.tar.zst
$ sudo mv nclip /usr/local/bin/nclip
$ sudo chmod +x /usr/local/bin/nclip
```

# Usage

```bash
$ nclip --help
Usage: nclip [OPTIONS]

Options:
  -c, --copy         copy from stdin
  -p, --port <PORT>  running port [default: 2230]
  -s, --server       running server
  -h, --help         Print help
  -V, --version      Print version
```

required: SSH remote port forwarding

## run server

```bash
$ nclip -s
```

## paste text from clipboard

```bash
$ nclip
```

## copy text to clipboard

```bash
$ echo "hello" | nclip -c
```

or

```bash
$ nclip -c < README.md
```

# NeoVim Configure

```vimrc
set clipboard+=unnamedplus
let g:clipboard = {
\   'name': 'nclip',
\   'copy': {
\      '+': 'nclip -c',
\      '*': 'nclip -c',
\    },
\   'paste': {
\      '+': 'nclip',
\      '*': 'nclip',
\   },
\   'cache_enabled': 1,
\ }
```
