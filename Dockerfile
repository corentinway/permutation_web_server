FROM alpine
WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/permutation_web_server .
CMD ["./permutation_web_server"]