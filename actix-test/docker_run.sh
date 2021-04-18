
docker run \
    --rm \
    -ti \
    -p $1:$1 \
    -v $(pwd):/workspace \
    -w /workspace \
    cjswosa22/rust-vision:0.1 \
    bash
