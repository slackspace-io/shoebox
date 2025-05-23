# Shoebox Documentation

Welcome to the Shoebox documentation! This guide will help you install, configure, and use Shoebox to organize and preserve your videos.

## What is Shoebox?

Shoebox is a digital solution for organizing and preserving your videos over a lifetime. It provides a dedicated space for your videos - a digital equivalent of that cherished shoebox in your closet.

## Getting Started

- [Introduction](./introduction.md) - Learn about the Shoebox concept and what makes it different
- [Installation](./installation.md) - Install Shoebox using Docker, Docker Compose, or Kubernetes
- [Configuration](./configuration.md) - Configure Shoebox to suit your needs
- [Usage](./usage.md) - Learn how to use Shoebox to organize and preserve your videos

## Key Features

- **Video organization and cataloging** - Keep your videos organized and easily searchable
- **Thumbnail generation** - Quickly identify videos with automatically generated thumbnails
- **Video metadata extraction** - Extract and use metadata from your videos
- **Export capabilities** - Export videos for use in external editing tools
- **Unreviewed videos workflow** - Efficiently process new videos
- **System information and management** - Monitor and manage your Shoebox installation

## Recent Enhancements

### Original Location Specification

A recent enhancement allows you to specify the original location of videos, which is useful when the path in your container or server differs from the original path where the videos were created or stored.

This feature is particularly useful for:
- Preserving metadata about the original location of videos
- Maintaining compatibility with external video editing tools
- Migrating videos between systems

See the [Configuration](./configuration.md) page for more details on how to use this feature.
