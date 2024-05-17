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

# Now create a small image to hold our binary and static files
FROM debian:buster-slim

# Install nginx
RUN apt-get update && apt-get install -y nginx && apt-get clean

# Copy the Rust binary
COPY --from=builder /bizzle/target/release/bizzle /usr/local/bin/bizzle

# Copy the static files
COPY ./static /usr/share/nginx/html

# Replace the default nginx config with our own
RUN rm /etc/nginx/sites-enabled/default
COPY nginx.conf /etc/nginx/sites-enabled/default

# Expose the port on which the app will run
EXPOSE 80

# Run nginx and the Rust binary
CMD ["sh", "-c", "nginx && bizzle"]
