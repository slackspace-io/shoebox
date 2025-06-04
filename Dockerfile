# Multi-stage build for Shoebox

# Stage 1: Build the frontend
FROM node:18-alpine AS frontend-builder
WORKDIR /app/frontend

# Copy frontend package.json and install dependencies
COPY frontend/package.json frontend/package-lock.json* ./
RUN yarn install

# Copy frontend source code
COPY frontend/ ./

# Build the frontend
RUN yarn run build

# Stage 2: Build the Rust backend
FROM rust:latest AS backend-builder
WORKDIR /app
# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    libavformat-dev \
    libavcodec-dev \
    libavutil-dev \
    libavfilter-dev \
    libswscale-dev \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./


# Copy actual source code
COPY src/ src/
COPY migrations/ migrations/

# Build the application
RUN cargo build --release

# Stage 3: Create the final image
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ffmpeg \
    ca-certificates \
    exiftool \
    && rm -rf /var/lib/apt/lists/*

# Copy the built frontend from stage 1
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

# Copy the built backend from stage 2
COPY --from=backend-builder /app/target/release/shoebox /app/shoebox

# Create directories for data
RUN mkdir -p /app/data /app/thumbnails /app/exports

# Set environment variables
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
#ENV DATABASE_URL=sqlite:/app/data/videos.db
ENV THUMBNAIL_PATH=/app/thumbnails
ENV EXPORT_BASE_PATH=/app/exports
ENV FRONTEND_PATH=/app/frontend/dist

# Expose the port
EXPOSE 3000

# Run the application
CMD ["/app/shoebox"]
