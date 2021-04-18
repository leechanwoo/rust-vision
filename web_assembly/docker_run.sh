
docker run \
    --rm \
    -ti \
    -p 8080:8080 \
    -v $(pwd):/workspace \
    -w /workspace \
    --name rust-vision \
    rust-vision \
    bash
