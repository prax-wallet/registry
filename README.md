# Penumbra Registry

The Penumbra registry hosts asset metadata, ibc info, and more configs.
It self-hosts data, but also fetches from the [cosmos asset registry](https://github.com/cosmos/chain-registry) to
augment ibc tokens with metadata.
Visit [the registry directory](../registry) for the output.

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
- The `input` directory should contain subdirectories named after each chain's ID, each of which should
  contain `ibc-assets.json` and `native-assets.json`.

Example:

```
input/
├── osmosis-test-5/
│   ├── ibc-assets.json
│   └── native-assets.json
└── mars-1/
    ├── ibc-assets.json
    └── native-assets.json
```

Each subdirectory should be named with the chain ID.

### Output Directory

The generated registry files will be saved to the `registry` directory within the project root. This directory is
created automatically if it does not exist.
