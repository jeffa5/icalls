# iCalLS: iCal language server

A tool to help with writing icalendar files.

## Actions

- [x] `hover` on properties
    - gradually adding more properties
- [x] completion for properties
    - gradually adding more properties
- [ ] diagnostics for unknown properties

## Installation

### Cargo

Currently, the main way to install icalls is by cloning the repo and running

```sh
cargo install --force icalls
```

This adds the binary `icalls` to the rust bin location.

### Nix

You can also get it on nix, using the flake in this repo:

```sh
nix shell github:jeffa5/icalls
```

## Configuration

Capabilities are all enabled by default, but can be disabled in the `initializationOptions` (e.g. to prevent conflicting handling of `hover` or `gotoDefinition`):

```json
{
  "enable_completion": false,
  "enable_hover": false,
  "enable_code_actions": false
}
```

### Neovim

For debugging and quickly adding it to neovim you can use the provided `vim.lua` file, provided you have `nvim-lspconfig`.
Just make sure to run `cargo build` and enter `nvim` from the root of this repo.

```sh
nvim test.eml
# then :LspStop
# then :luafile vim.lua
# then :LspStart
# Write some words and hit K to hover one
```

It by default is set up for the `icalendar` filetype.
