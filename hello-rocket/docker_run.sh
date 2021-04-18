
docker run \
    --rm \
    -ti \
    -p 8000:8000 \
    -v $(pwd):/workspace \
    -w /workspace \
    --name rust-vision \
    rust-vision \
    bash
