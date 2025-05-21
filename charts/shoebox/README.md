# Shoebox Helm Chart

This Helm chart deploys the Shoebox application on a Kubernetes cluster.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.2.0+
- PV provisioner support in the underlying infrastructure (if persistence is enabled)

## Getting Started

### Installing the Chart

To install the chart with the release name `shoebox`:

```bash
helm install shoebox .
```

### Using a Specific Image Version

By default, the chart uses the `latest` tag for the Shoebox image. For production environments, it's recommended to use a specific version:

```bash
helm install shoebox . --set image.tag=v1.0.0
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
helm install shoebox . --set imagePullSecrets[0].name=regcred
```

## Configuration

The following table lists the configurable parameters of the Shoebox chart and their default values.

### Image Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `image.repository` | Image repository | `ghcr.io/slackspace-io/shoebox` |
| `image.tag` | Image tag | `latest` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `imagePullSecrets` | Image pull secrets | `[]` |

### Application Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `config.serverHost` | Host to bind the server | `0.0.0.0` |
| `config.serverPort` | Port to bind the server | `3000` |
| `config.databaseUrl` | Database URL (SQLite) | `sqlite:/app/data/videos.db` |
| `config.mediaSourcePaths` | Paths to scan for videos | `/mnt/videos` |
| `config.thumbnailPath` | Path to store thumbnails | `/app/thumbnails` |
| `config.exportBasePath` | Path for exported files | `/app/exports` |
| `config.rustLog` | Rust log level | `info` |

### Persistence Configuration

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

### PostgreSQL Configuration

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
helm install shoebox . \
  --set persistence.data.enabled=true \
  --set persistence.thumbnails.enabled=true \
  --set persistence.exports.enabled=true \
  --set persistence.media.existingClaim=media-pvc
```

### Using PostgreSQL

```bash
helm install shoebox . \
  --set postgresql.enabled=true \
  --set postgresql.postgresqlPassword=mypassword \
  --set persistence.thumbnails.enabled=true \
  --set persistence.exports.enabled=true \
  --set persistence.media.existingClaim=media-pvc
```

### Disabling Persistence (for testing)

```bash
helm install shoebox . \
  --set persistence.data.enabled=false \
  --set persistence.thumbnails.enabled=false \
  --set persistence.exports.enabled=false \
  --set persistence.media.enabled=false
```

## Upgrading

### To 1.0.0

This is the first stable release of the Shoebox chart.
