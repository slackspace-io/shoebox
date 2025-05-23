# Installation

Shoebox can be installed in several ways, depending on your environment and preferences.

## Prerequisites

Before installing Shoebox, ensure you have the following prerequisites:

- [FFmpeg](https://ffmpeg.org/download.html) (for video processing)
- Access to storage for your videos, thumbnails, and exports

## Installation Methods

### Docker

The simplest way to run Shoebox is using Docker:

```bash
# Pull the latest image
docker pull ghcr.io/slackspace-io/shoebox:latest

# Run the container
docker run -d \
  -p 3000:3000 \
  -v /path/to/your/videos:/mnt/videos:ro \
  -v /path/to/your/exports:/app/exports \
  -v /path/to/thumbnails:/app/thumbnails \
  -v /path/to/data:/app/data \
  --name shoebox \
  ghcr.io/slackspace-io/shoebox:latest
```

### Docker Compose

For a more complete setup, you can use Docker Compose:

```bash
# Clone the repository
git clone https://github.com/slackspace-io/shoebox.git
cd shoebox

# Edit the docker-compose.yml file to configure your media source paths
# Start the application
docker-compose up -d
```

### Kubernetes with Helm

For Kubernetes deployments, Shoebox provides a Helm chart. See the [Helm Chart](./installation/helm-chart.md) page for detailed instructions.

### Development Setup

If you want to run Shoebox for development:

```bash
# Clone the repository
git clone https://github.com/slackspace-io/shoebox.git
cd shoebox

# Run the backend
cargo run

# In a separate terminal, run the frontend
cd frontend
yarn install
yarn dev
```

The frontend development server will be available at http://localhost:5173, and the backend server will be available at http://localhost:3000.
