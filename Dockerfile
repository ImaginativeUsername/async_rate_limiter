FROM debian as build

ARG PROTOC_VERSION="28.2"
ARG PROTOC_ARCH="x86_64"

RUN apt update \
    && apt upgrade -y \
    && apt install -y curl wget unzip mold gcc g++ make

RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

RUN wget "https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-${PROTOC_ARCH}.zip" -O ./protoc.zip
RUN unzip ./protoc.zip -x readme.txt -d /usr/local

WORKDIR /build
ADD ./protos ./protos
ADD ./src ./src
ADD build.rs Cargo.toml ./

RUN mold --run cargo build --release --features server_bin --bin server


FROM debian

RUN apt update && apt upgrade -y && apt install -y ca-certificates

WORKDIR /app
COPY --from=build /build/target/release/server ./run
ENTRYPOINT [ "./run" ]
