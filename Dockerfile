# Use the official Rust image from the Docker Hub
FROM rust:latest AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin bizzle
WORKDIR /bizzle

# Copy our existing project files
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

# Build for release
RUN cargo build --release

# Now create a small image to hold our binary
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev && apt-get clean
COPY --from=builder /bizzle/target/release/bizzle /usr/local/bin/bizzle

# We need to specify the port on which the app will run
ENV PORT 8080
EXPOSE 8080

# Run the binary
CMD ["bizzle"]
