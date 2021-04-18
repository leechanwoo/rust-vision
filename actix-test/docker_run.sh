
docker run \
    --rm \
    -ti \
    -p $1:$1 \
    -v $(pwd):/workspace \
    -w /workspace \
    rust-vision \
    bash
