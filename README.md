# Shoebox

A digital shoebox for organizing and preserving your videos over a lifetime.

## ⚠️ Active Development Notice

**This project is in active development and is not yet safe for production use.**

Features may change, data models might be restructured, and there could be bugs that affect your media files. Use at your own risk and always maintain backups of your original media.

## The Digital Shoebox Concept

Remember how previous generations kept their memories in physical shoeboxes at their parents' homes? Those boxes filled with photographs, negatives, and mementos that captured life's precious moments.

Shoebox aims to recreate that experience for the digital age. Instead of photos getting lost in the endless stream of cloud services or social media platforms, Shoebox provides a dedicated space for your videos - a digital equivalent of that cherished box in your closet.

## What Makes Shoebox Different

**Shoebox is not trying to compete with immich, Google Photos, or other photo management services.**

The main purpose of Shoebox is to help you:

- **Find original videos** export(copy) to a defined location, allowing you to easily import into a video editor of choice. Craete highlights, collages, etc. 
- **Organize your videos** over a lifetime for easy recall and future use. Have a coffee, review new videos cataloguing your memories as your kids grow. 
- **Preserve video memories** in a way that makes them accessible and workable

While other services focus on viewing and sharing, Shoebox focuses on organization and preservation with the specific goal of making your video content useful for future creative projects.

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Frontend**: React with TypeScript
- **Database**: SQLite/PostgreSQL via SQLx
- **Media Processing**: FFmpeg
- **Deployment**: Docker/Kubernetes support

## Features

- Video organization and cataloging
- Thumbnail generation
- Video metadata extraction
- Export capabilities
- Unreviewed videos workflow
- System information and management

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (for backend development)
- [Node.js](https://nodejs.org/) and [Yarn](https://yarnpkg.com/) (for frontend development)
- [FFmpeg](https://ffmpeg.org/download.html) (for video processing)
- [Docker](https://docs.docker.com/get-docker/) (optional, for containerized deployment)

### Running the Frontend (Development)

```bash
# Navigate to the frontend directory
cd frontend

# Install dependencies
yarn install

# Start the development server
yarn dev
```

The frontend development server will be available at http://localhost:5173.

### Running the Backend (Development)

```bash
# Run the backend server
cargo run
```

The backend server will be available at http://localhost:3000.

### Running with Docker

1. Edit the `docker-compose.yml` file to configure your media source paths:

```yaml
volumes:
  # Mount media source directories (read-only)
  - /path/to/your/videos:/mnt/videos:ro

  # Mount export directory (read-write)
  - /path/to/your/exports:/app/exports
```

2. Start the application:

```bash
docker-compose up -d
```

The application will be available at http://localhost:3000.

## Contributing

As this project is in active development, contributions are welcome but the codebase may change rapidly.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
