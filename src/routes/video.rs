use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router, response::{IntoResponse, Response}, http::{header, StatusCode, HeaderMap},
    body::Body,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use std::io::{SeekFrom, Seek};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use std::fs;
use std::io::Read;

use crate::error::{Result, AppError};
use crate::models::{CreateVideoDto, UpdateVideoDto, VideoSearchParams};
use crate::services::AppState;
use crate::services::VideoService;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(list_videos))
        .route("/", post(create_video))
        .route("/search", post(search_videos))
        .route("/{id}", get(get_video))
        .route("/{id}", put(update_video))
        .route("/{id}", delete(delete_video))
        .route("/{id}/stream", get(stream_video))
        .with_state(app_state)
}

#[derive(Debug, Deserialize)]
struct PaginationParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list_videos(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<crate::models::Video>>> {
    let video_service = VideoService::new(
        state.db.clone(),
        crate::services::TagService::new(state.db.clone()),
        crate::services::PersonService::new(state.db.clone()),
        crate::services::ThumbnailService::new(&state.config),
    );

    let limit = params.limit.unwrap_or(100);
    let offset = params.offset.unwrap_or(0);

    let videos = video_service.find_all(limit, offset).await?;
    Ok(Json(videos))
}

async fn get_video(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<crate::models::VideoWithMetadata>> {
    let video_service = VideoService::new(
        state.db.clone(),
        crate::services::TagService::new(state.db.clone()),
        crate::services::PersonService::new(state.db.clone()),
        crate::services::ThumbnailService::new(&state.config),
    );

    let video = video_service.find_with_metadata(&id).await?;
    Ok(Json(video))
}

async fn create_video(
    State(state): State<AppState>,
    Json(create_dto): Json<CreateVideoDto>,
) -> Result<Json<crate::models::Video>> {
    let video_service = VideoService::new(
        state.db.clone(),
        crate::services::TagService::new(state.db.clone()),
        crate::services::PersonService::new(state.db.clone()),
        crate::services::ThumbnailService::new(&state.config),
    );

    let video = video_service.create(create_dto).await?;
    Ok(Json(video))
}

async fn update_video(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(update_dto): Json<UpdateVideoDto>,
) -> Result<Json<crate::models::Video>> {
    let video_service = VideoService::new(
        state.db.clone(),
        crate::services::TagService::new(state.db.clone()),
        crate::services::PersonService::new(state.db.clone()),
        crate::services::ThumbnailService::new(&state.config),
    );

    let video = video_service.update(&id, update_dto).await?;
    Ok(Json(video))
}

async fn delete_video(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<()>> {
    let video_service = VideoService::new(
        state.db.clone(),
        crate::services::TagService::new(state.db.clone()),
        crate::services::PersonService::new(state.db.clone()),
        crate::services::ThumbnailService::new(&state.config),
    );

    video_service.delete(&id).await?;
    Ok(Json(()))
}

async fn search_videos(
    State(state): State<AppState>,
    Json(search_params): Json<VideoSearchParams>,
) -> Result<Json<Vec<crate::models::VideoWithMetadata>>> {
    let video_service = VideoService::new(
        state.db.clone(),
        crate::services::TagService::new(state.db.clone()),
        crate::services::PersonService::new(state.db.clone()),
        crate::services::ThumbnailService::new(&state.config),
    );

    let videos = video_service.search(search_params).await?;
    Ok(Json(videos))
}

async fn stream_video(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Response> {
    let video_service = VideoService::new(
        state.db.clone(),
        crate::services::TagService::new(state.db.clone()),
        crate::services::PersonService::new(state.db.clone()),
        crate::services::ThumbnailService::new(&state.config),
    );

    // Get the video to find its file path
    let video = video_service.find_by_id(&id).await?;

    // Check if the file exists
    let path = PathBuf::from(&video.file_path);
    if !path.exists() {
        return Err(AppError::NotFound(format!("Video file not found: {}", video.file_path)));
    }

    // Get the file size
    let metadata = match tokio::fs::metadata(&path).await {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Failed to read file metadata: {}", err)));
        }
    };

    let file_size = metadata.len();

    // Determine content type based on file extension
    let mut content_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    // Special handling for MP4 files to ensure metadata is properly positioned
    let is_mp4 = content_type == "video/mp4" || path.extension().map_or(false, |ext| ext == "mp4");

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
                    let mut file = match File::open(&path).await {
                        Ok(file) => file,
                        Err(err) => {
                            return Err(AppError::InternalServerError(format!("Failed to open video file: {}", err)));
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
    let file = match File::open(&path).await {
        Ok(file) => file,
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Failed to open video file: {}", err)));
        }
    };

    // For MP4 files, check if the moov atom is at the beginning of the file
    // If not, we need to add appropriate headers to help the browser
    let mut additional_headers = Vec::new();

    if is_mp4 {
        // Check MP4 file structure to see if moov atom is at the beginning
        let has_moov_at_beginning = check_mp4_structure(&path);

        // Add a header to force the browser to download the entire file before playing
        // This helps with MP4 files that have their metadata at the end
        additional_headers.push((header::ACCEPT_RANGES, "bytes".to_string()));

        // If moov atom is not at the beginning, add headers to help browsers handle it
        if !has_moov_at_beginning {
            // Keep the content type as video/mp4 to ensure proper decoding in browsers
            // Add Cache-Control: no-store to prevent caching of problematic MP4
            additional_headers.push((header::CACHE_CONTROL, "no-store".to_string()));

            // Add Content-Disposition: inline to suggest displaying the file inline
            additional_headers.push((header::CONTENT_DISPOSITION, "inline; filename=\"video.mp4\"".to_string()));

            // Add X-Content-Duration header with the duration if available
            // This can help some browsers with seeking
            if let Some(duration) = get_mp4_duration(&path) {
                additional_headers.push(("X-Content-Duration".parse().unwrap(), duration.to_string()));
            }

            // Add a header to indicate that the moov atom is at the end
            // This can help some browsers handle the file better
            additional_headers.push(("X-MP4-Has-Moov-At-Beginning".parse().unwrap(), "false".to_string()));
        } else {
            // For MP4 files with moov atom at the beginning, just add inline content disposition
            additional_headers.push((header::CONTENT_DISPOSITION, "inline".to_string()));
        }
    }

    // Create a stream from the file
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    // Build the response with proper headers for video streaming
    let mut response_builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, file_size)
        .header("X-Content-Type-Options", "nosniff");

    // Only add cache-control if it's not already in additional_headers
    if !additional_headers.iter().any(|(name, _)| name == &header::CACHE_CONTROL) {
        response_builder = response_builder.header(header::CACHE_CONTROL, "public, max-age=31536000");
    }

    // Add any additional headers for MP4 files
    for (name, value) in additional_headers {
        response_builder = response_builder.header(name, value);
    }

    let response = response_builder.body(body).unwrap();

    Ok(response)
}

// Function to check if an MP4 file has its moov atom at the beginning
fn check_mp4_structure(path: &PathBuf) -> bool {
    // Try to open the file in blocking mode for quick analysis
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return false, // If we can't open the file, assume the worst
    };

    // Read the first 1MB of the file to check for moov atom
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = [0u8; 8]; // 8 bytes for atom size (4) and type (4)
    let mut position = 0;

    // Limit our search to the first 1MB to avoid reading the entire file
    let search_limit = 1024 * 1024;

    while position < search_limit {
        // Read atom header
        match reader.read_exact(&mut buffer) {
            Ok(_) => {},
            Err(_) => break, // End of file or error
        }

        // Parse atom size (big-endian)
        let size = ((buffer[0] as u32) << 24) |
                  ((buffer[1] as u32) << 16) |
                  ((buffer[2] as u32) << 8) |
                  (buffer[3] as u32);

        // Check if this is the moov atom
        if &buffer[4..8] == b"moov" {
            return true; // Found moov atom near the beginning
        }

        // Skip to the next atom
        if size > 8 {
            // Skip the rest of this atom (size - 8 bytes we already read)
            let to_skip = size as u64 - 8;
            match reader.seek(SeekFrom::Current(to_skip as i64)) {
                Ok(new_pos) => position = new_pos,
                Err(_) => break, // Error seeking
            }
        } else if size == 0 {
            // Size 0 means the rest of the file, so we're done
            break;
        } else {
            // Invalid size, something is wrong
            break;
        }
    }

    // If we get here, we didn't find the moov atom near the beginning
    false
}

// Function to get the duration of an MP4 file (if possible)
fn get_mp4_duration(_path: &PathBuf) -> Option<f64> {
    // This is a simplified placeholder. In a real implementation,
    // you would parse the MP4 file to extract the actual duration.
    // For now, we'll just return None.
    None
}
