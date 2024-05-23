# # Use the official Rust image from the Docker Hub as the build environment
# FROM rust:latest AS builder

# # Create a new binary project in the /app directory
# RUN USER=root cargo new --bin bizzle
# WORKDIR /bizzle

# # Copy the Cargo.toml and Cargo.lock files to the build environment
# COPY ./Cargo.toml ./Cargo.lock ./

# # Copy the source files to the build environment
# COPY ./src ./src

# # Build the Rust project in release mode
# RUN cargo build --release

# # Use the same image for the run stage to ensure compatibility
# FROM ubuntu:20.04

# # Install nginx
# RUN apt-get update && apt-get install -y nginx && apt-get clean

# # Copy the compiled Rust binary from the builder stage
# COPY --from=builder /bizzle/target/release/bizzle /usr/local/bin/bizzle

# # Copy the static files for the frontend to the nginx html directory
# COPY ./static /usr/share/nginx/html

# # Copy the data files to the appropriate directory
# COPY ./data /usr/share/bizzle/data

# # Copy the nginx configuration file to the appropriate location
# COPY nginx.conf /etc/nginx/sites-available/default

# # Create a symbolic link to enable the nginx configuration
# # The -sf option forces the creation of the link, overwriting if it exists
# RUN ln -sf /etc/nginx/sites-available/default /etc/nginx/sites-enabled/default

# # Expose port 80 to be accessible outside the container
# EXPOSE 80

# # Command to run nginx and the Rust application, also tail nginx error log for debugging
# CMD ["sh", "-c", "nginx -g 'daemon off;' & bizzle & tail -f /var/log/nginx/error.log"]

# 1. This tells docker to use the Rust official image
FROM rust:latest

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/bizzle"]

