
FROM oza6ut0ne/opencv

RUN apt update
RUN apt install curl clang libclang-dev -y 

# install rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="${PATH}:/root/.cargo/bin"

# instal cargo-generate
RUN cargo install cargo-generate

# install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# install nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash 
ENV NVM_DIR="/root/.nvm"

# install nodejs
RUN bash -c ". /root/.nvm/nvm.sh && nvm install 14.4.0"


