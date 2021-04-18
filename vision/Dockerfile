
FROM oza6ut0ne/opencv
#FROM valian/docker-python-opencv-ffmpeg:py3
#FROM spmallick/opencv-3.4.3

RUN apt update
RUN apt install curl clang libclang-dev -y 

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="${PATH}:/root/.cargo/bin"
#ENV PKG_CONFIG_PATH="/usr/local/lib/pkgconfig"
#ENV LD_LIBRARY_PATH="/usr/local/lib"

