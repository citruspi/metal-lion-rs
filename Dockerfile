FROM rust

RUN mkdir /source

COPY assets /source/assets
COPY src /source/src
COPY Cargo.toml /source/Cargo.toml
COPY Cargo.lock /source/Cargo.lock

RUN cd /source && cargo build --release

FROM debian:stable

COPY --from=0 /source/target/release/metal-lion /bin/metal-lion

ENTRYPOINT ["/bin/metal-lion"]