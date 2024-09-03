# Update registry info JSON files
run:
  cd tools/compiler && \
    RUST_LOG=penumbra_registry=debug cargo run
