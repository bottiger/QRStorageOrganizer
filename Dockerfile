FROM ekidd/rust-musl-builder as builder

WORKDIR /home/rust/src
COPY --chown=rust ./Cargo* ./
COPY --chown=rust ./src ./src/

RUN cargo build --bin rest --release

FROM alpine:latest AS app
# Set working directory
# Copy in statically linked binary from builder stage
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rest /
# Expose port for server
EXPOSE 8088
# Run entrypoint script
ENTRYPOINT ["/rest"]
