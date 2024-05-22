# Use the official Rust image from the Docker Hub
FROM rust:latest AS builder

# Create a new binary project in /app
RUN USER=root cargo new --bin bizzle
WORKDIR /bizzle

# Copy the Cargo.toml and Cargo.lock files
COPY ./Cargo.toml ./Cargo.lock ./

# Copy the source files
COPY ./src ./src

# Build for release
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:buster-slim

# Install nginx
RUN apt-get update && apt-get install -y nginx && apt-get clean

# Copy the Rust binary from the builder stage
COPY --from=builder /bizzle/target/release/bizzle /usr/local/bin/bizzle

# Copy the static files
COPY ./static /usr/share/nginx/html

# Copy the data file
COPY ./data /usr/share/bizzle/data

# Copy the nginx config
COPY nginx.conf /etc/nginx/sites-available/default
RUN ln -sf /etc/nginx/sites-available/default /etc/nginx/sites-enabled/default

# Expose the port on which the app will run
EXPOSE 80

# Run nginx and the Rust binary
CMD ["sh", "-c", "nginx && bizzle"]
