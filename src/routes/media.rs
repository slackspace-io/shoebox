use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use axum::body::Body;

use crate::error::{AppError, Result};
use crate::services::AppState;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/{*path}", get(serve_media))
        .with_state(app_state)
}

async fn serve_media(
    State(state): State<AppState>,
    Path(path): Path<String>,
    headers: HeaderMap,
) -> Result<Response> {
    // Construct the full path to the media file
    let base_path = PathBuf::from(&state.config.media.source_paths[0]);
    let file_path = base_path.join(&path);

    // Check if the file exists
    if !file_path.exists() {
        return Err(AppError::NotFound(format!("Media file not found: {}", path)));
    }

    // Get the file size
    let metadata = match tokio::fs::metadata(&file_path).await {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Failed to read file metadata: {}", err)));
        }
    };

    let file_size = metadata.len();

    // Determine content type based on file extension
    let content_type = match file_path.extension().and_then(|ext| ext.to_str()) {
        Some("mov") => "video/quicktime".to_string(),
        Some("mp4") => "video/mp4".to_string(),
        Some("mkv") => "video/x-matroska".to_string(),
        _ => mime_guess::from_path(&file_path)
            .first_or_octet_stream()
            .to_string()
    };

    // Get range header if it exists
    let range_header = headers.get(header::RANGE);

    // Handle range request if present
    if let Some(range) = range_header {
        // Parse the range header
        let range_str = range.to_str().map_err(|_| {
            AppError::BadRequest("Invalid range header".to_string())
        })?;

        // Parse range values (format: "bytes=start-end")
        if let Some(range_values) = range_str.strip_prefix("bytes=") {
            let ranges: Vec<&str> = range_values.split('-').collect();
            if ranges.len() == 2 {
                let start = ranges[0].parse::<u64>().unwrap_or(0);
                let end = ranges[1].parse::<u64>().unwrap_or(file_size - 1).min(file_size - 1);

                // Ensure start is less than end and within file bounds
                if start <= end && start < file_size {
                    let length = end - start + 1;

                    // Open the file
                    let mut file = match File::open(&file_path).await {
                        Ok(file) => file,
                        Err(err) => {
                            return Err(AppError::InternalServerError(format!("Failed to open media file: {}", err)));
                        }
                    };

                    // Seek to the start position
                    use tokio::io::AsyncSeekExt;
                    if let Err(err) = file.seek(std::io::SeekFrom::Start(start)).await {
                        return Err(AppError::InternalServerError(format!("Failed to seek in file: {}", err)));
                    }

                    // Create a limited stream from the file
                    use tokio::io::AsyncReadExt;
                    let stream = ReaderStream::new(file.take(length));
                    let body = Body::from_stream(stream);

                    // Build the response with partial content status
                    let response = Response::builder()
                        .status(StatusCode::PARTIAL_CONTENT)
                        .header(header::CONTENT_TYPE, content_type)
                        .header(header::CONTENT_LENGTH, length)
                        .header(header::CONTENT_RANGE, format!("bytes {}-{}/{}", start, end, file_size))
                        .header(header::ACCEPT_RANGES, "bytes")
                        .header(header::CACHE_CONTROL, "public, max-age=31536000")
                        .header("X-Content-Type-Options", "nosniff")
                        .body(body)
                        .unwrap();

                    return Ok(response);
                }
            }
        }

        // If we get here, the range was invalid
        return Err(AppError::BadRequest("Invalid range format".to_string()));
    }

    // If no range header or parsing failed, serve the entire file
    // Open the file
    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Failed to open media file: {}", err)));
        }
    };

    // Create a stream from the file
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    // Build the response with proper headers for media streaming
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, file_size)
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CACHE_CONTROL, "public, max-age=31536000")
        .header("X-Content-Type-Options", "nosniff")
        .body(body)
        .unwrap();

    Ok(response)
}
