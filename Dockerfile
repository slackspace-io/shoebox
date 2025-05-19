# Multi-stage build for Family Video Organizer

# Stage 1: Build the frontend
FROM node:18-alpine as frontend-builder
WORKDIR /app/frontend

# Copy frontend package.json and install dependencies
COPY frontend/package.json frontend/package-lock.json* ./
RUN npm install

# Copy frontend source code
COPY frontend/ ./

# Build the frontend
RUN npm run build

# Stage 2: Build the Rust backend
FROM rust:1.70-slim as backend-builder
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

# Create dummy src/main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src/ src/
#COPY migrations/ migrations/

# Build the application
RUN cargo build --release

# Stage 3: Create the final image
FROM debian:bullseye-slim
WORKDIR /app

RUN apt search libavcodec
# Install runtime dependencies
#RUN apt-get update && apt-get install -y \
#    libsqlite3-0 \
#    libavformat59 \
#    libavcodec57 \
#    libavutil56 \
#    libavfilter8 \
#    libswscale6 \
#    ffmpeg \
#    ca-certificates \
#    && rm -rf /var/lib/apt/lists/*

# Copy the built frontend from stage 1
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

# Copy the built backend from stage 2
COPY --from=backend-builder /app/target/release/family_video_organizer /app/family_video_organizer

# Create directories for data
RUN mkdir -p /app/data /app/thumbnails /app/exports

# Set environment variables
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
ENV DATABASE_URL=sqlite:/app/data/videos.db
ENV THUMBNAIL_PATH=/app/thumbnails
ENV EXPORT_BASE_PATH=/app/exports

# Expose the port
EXPOSE 3000

# Run the application
CMD ["/app/family_video_organizer"]
