# Helm Chart Installation

This page provides detailed instructions for deploying Shoebox on Kubernetes using the Helm chart.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.2.0+
- PV provisioner support in the underlying infrastructure (if persistence is enabled)

## Getting Started

### Adding the Helm Repository

```bash
# Add the Shoebox Helm repository
helm repo add shoebox https://slackspace-io.github.io/shoebox
helm repo update
```

### Installing the Chart

To install the chart with the release name `shoebox`:

```bash
helm install shoebox shoebox/shoebox
```

### Using a Specific Image Version

By default, the chart uses the `preview` tag for the Shoebox image. For production environments, it's recommended to use a specific version:

```bash
helm install shoebox shoebox/shoebox --set image.tag=v1.0.0
```

### Using a Private Registry

If you're using a private registry for the Shoebox image, you'll need to create a secret with your registry credentials:

```bash
kubectl create secret docker-registry regcred \
  --docker-server=ghcr.io \
  --docker-username=<your-username> \
  --docker-password=<your-token> \
  --docker-email=<your-email>
```

Then, specify the secret in your Helm install command:

```bash
helm install shoebox shoebox/shoebox --set imagePullSecrets[0].name=regcred
```

## Configuration

The Shoebox Helm chart offers extensive configuration options through its `values.yaml` file. You can override these values using the `--set` flag or by providing your own values file.

### Media Source Paths Configuration

One of the key features of Shoebox is the ability to specify the original location of videos. This is configured through the `config.mediaSourcePaths` parameter.

The `mediaSourcePaths` parameter accepts a comma-separated list of paths. Each path can be configured in two formats:

#### Named Section Format (Recommended)

```
name:/path/to/videos;/original/path;original_extension
```

Where:
- `name` is a descriptive name for the media source (e.g., "bmpcc", "gopro", etc.)
- `/path/to/videos` is the path where the videos are mounted in the container (required)
- `/original/path` (optional) is the original location of the videos on the source system
- `original_extension` (optional) is the original file extension of the videos. If not provided but `original_path` is, it will use the same extension as the scan path.

For example:

```yaml
config:
  mediaSourcePaths: "bmpcc:/mnt/videos;/home/user/videos;mp4,gopro:/mnt/other-videos;/media/external/videos"
```

For better readability, you can also use YAML's multi-line string syntax:

```yaml
config:
  mediaSourcePaths: >-
    bmpcc:/mnt/videos;/home/user/videos;mp4,
    gopro:/mnt/other-videos;/media/external/videos
```

Both configurations specify two named media source paths:
1. `bmpcc` with scan path `/mnt/videos`, original path `/home/user/videos`, and original extension `mp4`
2. `gopro` with scan path `/mnt/other-videos`, original path `/media/external/videos`, and using the same extension as the scan path

#### Legacy Format (Backward Compatible)

The older format without named sections is still supported:

```
/path/to/videos;/original/path;original_extension
```

For example:

```yaml
config:
  mediaSourcePaths: "/mnt/videos;/home/user/videos;mp4,/mnt/other-videos;/media/external/videos"
```

You can set this configuration when installing the chart:

```bash
helm install shoebox shoebox/shoebox \
  --set config.mediaSourcePaths="/mnt/videos;/home/user/videos;mp4,/mnt/other-videos;/media/external/videos"
```

### Other Configuration Parameters

#### Image Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `image.repository` | Image repository | `ghcr.io/slackspace-io/shoebox` |
| `image.tag` | Image tag | `preview` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `imagePullSecrets` | Image pull secrets | `[]` |

#### Application Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `config.serverHost` | Host to bind the server | `0.0.0.0` |
| `config.serverPort` | Port to bind the server | `3000` |
| `config.databaseUrl` | Database URL (SQLite) | `sqlite:/app/data/videos.db` |
| `config.mediaSourcePaths` | Paths to scan for videos | `/mnt/videos` |
| `config.thumbnailPath` | Path to store thumbnails | `/app/thumbnails` |
| `config.exportBasePath` | Path for exported files | `/app/exports` |
| `config.rustLog` | Rust log level | `info` |

#### Persistence Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `persistence.data.enabled` | Enable persistence for data | `true` |
| `persistence.data.size` | Size of data PVC | `1Gi` |
| `persistence.thumbnails.enabled` | Enable persistence for thumbnails | `true` |
| `persistence.thumbnails.size` | Size of thumbnails PVC | `5Gi` |
| `persistence.exports.enabled` | Enable persistence for exports | `true` |
| `persistence.exports.size` | Size of exports PVC | `10Gi` |
| `persistence.media.enabled` | Enable persistence for media | `true` |
| `persistence.media.existingClaim` | Use existing PVC for media | `""` |
| `persistence.media.size` | Size of media PVC | `100Gi` |

#### PostgreSQL Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `postgresql.enabled` | Enable PostgreSQL | `false` |
| `postgresql.postgresqlUsername` | PostgreSQL username | `postgres` |
| `postgresql.postgresqlPassword` | PostgreSQL password | `postgres` |
| `postgresql.postgresqlDatabase` | PostgreSQL database | `videos` |
| `postgresql.persistence.enabled` | Enable PostgreSQL persistence | `true` |
| `postgresql.persistence.size` | Size of PostgreSQL PVC | `8Gi` |

## Examples

### Using SQLite with Persistence

```bash
helm install shoebox shoebox/shoebox \
  --set persistence.data.enabled=true \
  --set persistence.thumbnails.enabled=true \
  --set persistence.exports.enabled=true \
  --set persistence.media.existingClaim=media-pvc
```

### Using PostgreSQL

```bash
helm install shoebox shoebox/shoebox \
  --set postgresql.enabled=true \
  --set postgresql.postgresqlPassword=mypassword \
  --set persistence.thumbnails.enabled=true \
  --set persistence.exports.enabled=true \
  --set persistence.media.existingClaim=media-pvc
```

### Configuring Multiple Media Source Paths with Original Locations

```bash
# Using a single line
helm install shoebox shoebox/shoebox \
  --set config.mediaSourcePaths="bmpcc:/mnt/videos;/home/user/videos;mp4,gopro:/mnt/other-videos;/media/external/videos" \
  --set persistence.thumbnails.enabled=true \
  --set persistence.exports.enabled=true \
  --set persistence.media.existingClaim=media-pvc

# Or using a values file with the multi-line syntax for better readability
cat > values-custom.yaml << EOF
config:
  mediaSourcePaths: >-
    bmpcc:/mnt/videos;/home/user/videos;mp4,
    gopro:/mnt/other-videos;/media/external/videos
persistence:
  thumbnails:
    enabled: true
  exports:
    enabled: true
  media:
    existingClaim: media-pvc
EOF

helm install shoebox shoebox/shoebox -f values-custom.yaml
```

### Disabling Persistence (for testing)

```bash
helm install shoebox shoebox/shoebox \
  --set persistence.data.enabled=false \
  --set persistence.thumbnails.enabled=false \
  --set persistence.exports.enabled=false \
  --set persistence.media.enabled=false
```

## Upgrading

### To 1.0.0

This is the first stable release of the Shoebox chart.
