# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y npm nodejs

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Install necessary tools
RUN rustup toolchain install nightly --allow-downgrade && \
    rustup target add wasm32-unknown-unknown && \
    cargo install cargo-leptos --locked && \
    npm install -g sass

# Build the project
RUN cargo leptos build --release

# Use a minimal image for the final stage
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the server binary and site files from the builder stage
COPY --from=builder /app/target/release/shoebox /app/target/server/release/shoebox
COPY --from=builder /app/target/site /app/target/site

# Set environment variables
ENV LEPTOS_OUTPUT_NAME="shoebox"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_RELOAD_PORT="3001"

# Expose the port
EXPOSE 8080

# Run the server binary
CMD ["/app/target/server/release/shoebox"]
