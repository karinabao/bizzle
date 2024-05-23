# Use a musl-based Rust image for static linking
FROM ekidd/rust-musl-builder AS builder

# Create a new binary project in the /bizzle directory
RUN USER=root cargo new --bin bizzle
WORKDIR /home/rust/src/bizzle

# Copy the Cargo.toml and Cargo.lock files to the build environment
COPY ./Cargo.toml ./Cargo.lock ./

# Fetch dependencies
RUN cargo build --release

# Now copy the actual source code
COPY ./src ./src

# Rebuild the project with the actual source code
RUN cargo build --release --target x86_64-unknown-linux-musl

# Use a minimal base image for the runtime stage
FROM scratch

# Copy the statically linked Rust binary from the builder stage
COPY --from=builder /home/rust/src/bizzle/target/x86_64-unknown-linux-musl/release/bizzle /usr/local/bin/bizzle

# Copy the static files for the frontend to the nginx html directory
COPY ./static /usr/share/nginx/html

# Copy the data files to the appropriate directory
COPY ./data /usr/share/bizzle/data

# Expose port 80 to be accessible outside the container
EXPOSE 80

# Command to run the Rust application
CMD ["/usr/local/bin/bizzle"]
