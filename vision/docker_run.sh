
docker run \
    --rm \
    -ti \
    -v $(pwd):/workspace \
    -w /workspace \
    --name rust-vision \
    rust-vision \
    bash
