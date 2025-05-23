# Configuration

Shoebox offers various configuration options to customize its behavior according to your needs.

## Environment Variables

Shoebox can be configured using environment variables. Here are the main configuration options:

### Server Configuration

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `SERVER_HOST` | Host to bind the server | `127.0.0.1` |
| `SERVER_PORT` | Port to bind the server | `3000` |

### Database Configuration

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `DATABASE_URL` | Database connection URL | `sqlite:data.db` |
| `DATABASE_MAX_CONNECTIONS` | Maximum number of database connections | `5` |

### Media Configuration

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `MEDIA_SOURCE_PATHS` | Paths to scan for videos | `./media` |
| `EXPORT_BASE_PATH` | Path for exported files | `./exports` |
| `THUMBNAIL_PATH` | Path to store thumbnails | `./thumbnails` |

## Media Source Paths Configuration

The `MEDIA_SOURCE_PATHS` environment variable is particularly important as it defines where Shoebox looks for videos. This variable accepts a comma-separated list of paths.

### Basic Usage

For basic usage, you can specify one or more paths:

```
MEDIA_SOURCE_PATHS=/path/to/videos,/path/to/more/videos
```

### Advanced Configuration with Original Locations

A recent enhancement allows you to specify the original location of videos, which is useful when the path in your container or server differs from the original path where the videos were created or stored.

Each path can include additional configuration options using the following format:

```
/path/to/videos;/original/path;original_extension
```

Where:
- `/path/to/videos` is the path where the videos are mounted in the container or server
- `/original/path` (optional) is the original location of the videos on the source system
- `original_extension` (optional) is the original file extension of the videos

For example:

```
MEDIA_SOURCE_PATHS=/mnt/videos;/home/user/videos;mp4,/mnt/other-videos;/media/external/videos
```

This configuration specifies two media source paths:
1. `/mnt/videos` with original path `/home/user/videos` and original extension `mp4`
2. `/mnt/other-videos` with original path `/media/external/videos` and no specific extension

### Why Specify Original Locations?

Specifying the original location of videos is useful for several reasons:

1. **Preserving metadata**: When exporting videos, Shoebox can include information about their original location, which helps with organization and traceability.
2. **Compatibility with external tools**: Some video editing tools may use absolute paths in project files. By knowing the original path, Shoebox can help maintain compatibility.
3. **Migration between systems**: If you move your videos from one system to another, specifying the original location helps maintain consistency in your workflow.

## Configuration Files

Currently, Shoebox does not support configuration files directly. All configuration is done through environment variables or command-line arguments.

For Kubernetes deployments using the Helm chart, configuration is done through the `values.yaml` file or by setting values with the `--set` flag. See the [Helm Chart](./installation/helm-chart.md) page for more details.
