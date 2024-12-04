# Use a Rust base image with Cargo installed
FROM rust:1.83.0 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to trick Cargo into thinking it's a valid Rust project
RUN mkdir src && touch src/lib.rs

# cache dependencies
RUN cargo build --release

RUN rm -rf src

# Now copy the source code
COPY ./src ./src

# Build your application
RUN cargo build --bin rest --release

EXPOSE 8080
CMD ["./target/release/rest"]