# Use the official Rust image from the Docker Hub as the build environment
FROM rust:latest AS builder

# Create a new binary project in the /app directory
RUN USER=root cargo new --bin bizzle
WORKDIR /bizzle

# Copy the Cargo.toml and Cargo.lock files to the build environment
COPY ./Cargo.toml ./Cargo.lock ./

# Copy the source files to the build environment
COPY ./src ./src

# Build the Rust project in release mode
RUN cargo build --release

# Use a more up-to-date base image with the required glibc version
FROM debian:bullseye-slim

# Install nginx
RUN apt-get update && apt-get install -y nginx && apt-get clean

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /bizzle/target/release/bizzle /usr/local/bin/bizzle

# Copy the static files for the frontend to the nginx html directory
COPY ./static /usr/share/nginx/html

# Copy the data files to the appropriate directory
COPY ./data /usr/share/bizzle/data

# Copy the nginx configuration file to the appropriate location
COPY nginx.conf /etc/nginx/sites-available/default

# Create a symbolic link to enable the nginx configuration
# The -sf option forces the creation of the link, overwriting if it exists
RUN ln -sf /etc/nginx/sites-available/default /etc/nginx/sites-enabled/default

# Expose port 80 to be accessible outside the container
EXPOSE 80

# Command to run nginx and the Rust application, also tail nginx error log for debugging
CMD ["sh", "-c", "nginx -g 'daemon off;' & bizzle & tail -f /var/log/nginx/error.log"]
