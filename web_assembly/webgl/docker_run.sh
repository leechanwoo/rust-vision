
docker run \
    --rm \
    -ti \
    -p 8080:8080 \
    -v $(pwd):/workspace \
    -w /workspace \
    --name rust-webgl \
    rust-vision \
    bash
