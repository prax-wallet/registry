# Prax Chain Registry

This repo contains a registry of metadata for assets on Penumbra.

Penumbra records value in its shielded pool tagged by _asset ID_, and client
software must map asset IDs to a `Metadata` object describing its base denom,
display units, symbol, asset icons, etc. Because this information is presented
to users to help them understand their actions, it is relatively
security-critical. As in other parts of the Cosmos ecosystem, the expectation
for the Penumbra ecosystem is that client software is responsible for choosing
how to display asset metadata.

The Prax chain registry is the compilation of those choices for Prax wallet,
though it may also be reused by other ecosystem tooling. It presents a view of
asset metadata derived from both the Cosmos chain registry and its own data.
In particular, each IBC asset transferred into Penumbra will have its own
`Metadata`. The compiler in this repo uses a list of known channels to produce
derived metadata for IBC assets on Penumbra based on the definitions in the
Cosmos chain registry.

### Why not use the Cosmos Chain Registry?

We do, but indirectly. The Cosmos chain registry contains data about assets on
Cosmos chains. Penumbra clients require data about assets on Penumbra,
including Penumbra-specific data such as the Penumbra asset ID. The Prax
registry is a registry of Cosmos assets _on Penumbra_.

We also intend to submit metadata about Penumbra assets upstream to the CCR for
use by other Cosmos chains.

### IBC Channels

The Prax view of which IBC channels are canonical is set by the `ibcConnections`
field in the input JSON file.

## Generator usage

Let's say you want to add or edit an asset in the registry. Here are the steps you should take.

1. Install pre-requisite deps: [Rust](https://www.rust-lang.org/tools/install) & [pnpm](https://pnpm.io/installation)
2. Fork repository & clone it down locally with `git clone --recurse-submodules`
    - If you've pulled the repo before, update the cosmos registry using `git submodule update --init --recursive`
3. In `input/<chain_name>.json`, add your new asset or edit the metadata as needed
4. In `tools/compiler/` directory run `cargo run`
5. In `npm/` run `pnpm install` and `pnpm changeset`. Make a minor (if new chain added) or patch (add/edit to existing)
   update. It will ask for a summary of the change.
6. In `npm/` run `pnpm changeset version`. This will bump the package version.
7. Make PR. Maintainers will review and merge.
8. After merge, the latest registry will be live in `registry/` and available in the newly published npm package
   version.

### File structure

The generator expects a specific directory structure:

- An `input` directory at the root of the project, containing configuration JSON files for each chain you want to
  process.

Example:

```
input/
├── penumbra-testnet-deimos-6.json
└── mars-1.json
```

Each file should be named with the chain ID.

### Output Directory

The generated registry files will be saved to the `registry` directory within the project root. This directory is
created automatically if it does not exist.
