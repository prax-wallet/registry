# update registry info JSON files
assets:
  cd tools/compiler && \
    RUST_LOG=penumbra_registry=debug cargo run

# build the npm package from registry JSON files
build:
  cd npm && \
    pnpm build && \
    npm pack
