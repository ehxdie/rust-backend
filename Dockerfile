# Use the official Rust image as a parent image
FROM rust:1.76 as builder

# Set the working directory in the container
WORKDIR /usr/src/rust-backend

# Copy the entire project
COPY . .

# Build the application
RUN cargo build --release

# Use the same Rust image for the runtime stage
FROM rust:1.76-slim

# Install OpenSSL - required for some Rust applications
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/rust-backend/target/release/rust-backend /usr/local/bin/rust-backend

EXPOSE 8080

# Set the startup command
CMD ["rust-backend"]