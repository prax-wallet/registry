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

## Generator usage

To generate a fresh registry, you must have Rust and Cargo installed. Once you have cloned the repository and navigated
to the root directory of the project, you can run the application with the following command:

```bash
cargo run
```

If there are changes, create a PR and push them up.

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
