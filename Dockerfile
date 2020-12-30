FROM debian:stable

COPY target/release/metal-lion /bin/metal-lion

ENTRYPOINT ["/bin/metal-lion"]