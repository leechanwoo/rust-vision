
docker run \
    --rm \
    -v $(pwd):/workspace \
    -w /workspace \
    --name rust-vision \
    rust-vision \
    cargo new $1
