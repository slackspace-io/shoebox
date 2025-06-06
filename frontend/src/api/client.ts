import axios from 'axios';

// Define base URL for API
const API_URL = '/api';

// Create axios instance
const apiClient = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Types
export interface Video {
  id: string;
  file_path: string;
  file_name: string;
  title?: string;
  description?: string;
  created_date?: string;
  file_size?: number;
  thumbnail_path?: string;
  rating?: number;
  duration?: number;
  location?: string;
  event?: string;
  created_at: string;
  updated_at: string;
}

export interface VideoWithMetadata extends Video {
  tags: string[];
  people: string[];
  shoeboxes: string[];
}

export interface CreateVideoDto {
  file_path: string;
  file_name: string;
  title?: string;
  description?: string;
  created_date?: string;
  file_size?: number;
  thumbnail_path?: string;
  rating?: number;
  location?: string;
  event?: string;
  tags: string[];
  people: string[];
}

export interface UpdateVideoDto {
  title?: string;
  description?: string;
  rating?: number;
  location?: string;
  event?: string;
  tags?: string[];
  people?: string[];
  shoeboxes?: string[];
}

export interface BulkUpdateVideoDto {
  video_ids: string[];
  update: UpdateVideoDto;
}

export interface VideoSearchParams {
  query?: string;
  tags?: string[];
  people?: string[];
  location?: string;
  event?: string;
  rating?: number;
  limit?: number;
  offset?: number;
  unreviewed?: boolean;
  sort_by?: string;
  sort_order?: string;
  start_date?: string;
  end_date?: string;
  min_duration?: number;
  max_duration?: number;
}

export interface Tag {
  id: string;
  name: string;
  created_at: string;
}

export interface TagUsage {
  id: string;
  name: string;
  video_count: number;
}

export interface Person {
  id: string;
  name: string;
  created_at: string;
}

export interface PersonUsage {
  id: string;
  name: string;
  video_count: number;
}

export interface LocationUsage {
  name: string;
  video_count: number;
}

export interface EventUsage {
  name: string;
  video_count: number;
}

export interface Shoebox {
  id: string;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

export interface ShoeboxUsage {
  id: string;
  name: string;
  description?: string;
  video_count: number;
}

export interface ExportRequest {
  video_ids: string[];
  project_name: string;
  use_original_files?: boolean;
}

export interface ExportResponse {
  export_path: string;
  video_count: number;
}

export interface ScanResponse {
  new_videos_count: number;
  new_videos: Video[];
}

// API functions
export const videoApi = {
  // Get all videos with pagination
  getVideos: async (limit = 100, offset = 0): Promise<Video[]> => {
    const response = await apiClient.get(`/videos?limit=${limit}&offset=${offset}`);
    return response.data;
  },

  // Get a single video by ID
  getVideo: async (id: string): Promise<VideoWithMetadata> => {
    const response = await apiClient.get(`/videos/${id}`);
    return response.data;
  },

  // Update a video
  updateVideo: async (id: string, data: UpdateVideoDto): Promise<VideoWithMetadata> => {
    const response = await apiClient.put(`/videos/${id}`, data);
    return response.data;
  },

  // Bulk update multiple videos
  bulkUpdateVideos: async (data: BulkUpdateVideoDto): Promise<Video[]> => {
    const response = await apiClient.post('/videos/bulk-update', data);
    return response.data;
  },

  // Delete a video
  deleteVideo: async (id: string): Promise<void> => {
    await apiClient.delete(`/videos/${id}`);
  },

  // Search videos
  searchVideos: async (params: VideoSearchParams): Promise<VideoWithMetadata[]> => {
    const response = await apiClient.post('/videos/search', params);
    return response.data;
  },
};

export const tagApi = {
  // Get all tags
  getTags: async (): Promise<Tag[]> => {
    const response = await apiClient.get('/tags');
    return response.data;
  },

  // Get tag usage statistics
  getTagUsage: async (): Promise<TagUsage[]> => {
    const response = await apiClient.get('/tags/usage');
    return response.data;
  },

  // Delete unused tags
  deleteUnusedTags: async (): Promise<void> => {
    await apiClient.delete('/tags/unused');
  },

  // Create a new tag
  createTag: async (name: string): Promise<Tag> => {
    const response = await apiClient.post('/tags', { name });
    return response.data;
  },

  // Delete a tag by ID
  deleteTag: async (id: string): Promise<void> => {
    await apiClient.delete(`/tags/${id}`);
  },
};

export const personApi = {
  // Get all people
  getPeople: async (): Promise<Person[]> => {
    const response = await apiClient.get('/people');
    return response.data;
  },

  // Get person usage statistics
  getPersonUsage: async (): Promise<PersonUsage[]> => {
    const response = await apiClient.get('/people/usage');
    return response.data;
  },

  // Delete unused people
  deleteUnusedPeople: async (): Promise<void> => {
    await apiClient.delete('/people/unused');
  },

  // Create a new person
  createPerson: async (name: string): Promise<Person> => {
    const response = await apiClient.post('/people', { name });
    return response.data;
  },

  // Delete a person by ID
  deletePerson: async (id: string): Promise<void> => {
    await apiClient.delete(`/people/${id}`);
  },
};

export const locationApi = {
  // Get all locations
  getLocations: async (): Promise<string[]> => {
    const response = await apiClient.get('/locations');
    return response.data;
  },

  // Get location usage statistics
  getLocationUsage: async (): Promise<LocationUsage[]> => {
    const response = await apiClient.get('/locations/usage');
    return response.data;
  },

  // Update a location
  updateLocation: async (oldLocation: string, newLocation: string): Promise<number> => {
    const response = await apiClient.post('/locations/update', {
      old_location: oldLocation,
      new_location: newLocation
    });
    return response.data;
  },

  // Delete a location
  deleteLocation: async (location: string): Promise<number> => {
    const response = await apiClient.delete(`/locations/${encodeURIComponent(location)}`);
    return response.data;
  },
};

export const eventApi = {
  // Get all events
  getEvents: async (): Promise<string[]> => {
    const response = await apiClient.get('/events');
    return response.data;
  },

  // Get event usage statistics
  getEventUsage: async (): Promise<EventUsage[]> => {
    const response = await apiClient.get('/events/usage');
    return response.data;
  },

  // Update an event
  updateEvent: async (oldEvent: string, newEvent: string): Promise<number> => {
    const response = await apiClient.post('/events/update', {
      old_event: oldEvent,
      new_event: newEvent
    });
    return response.data;
  },

  // Delete an event
  deleteEvent: async (event: string): Promise<number> => {
    const response = await apiClient.delete(`/events/${encodeURIComponent(event)}`);
    return response.data;
  },
};

export const scanApi = {
  // Scan directories for new videos
  scanDirectories: async (): Promise<ScanResponse> => {
    const response = await apiClient.post('/scan');
    return response.data;
  },
};

export const exportApi = {
  // Export videos
  exportVideos: async (data: ExportRequest): Promise<ExportResponse> => {
    const response = await apiClient.post('/export', data);
    return response.data;
  },
};

export const shoeboxApi = {
  // Get all shoeboxes
  getShoeboxes: async (): Promise<Shoebox[]> => {
    const response = await apiClient.get('/shoeboxes');
    return response.data;
  },

  // Get shoebox usage statistics
  getShoeboxUsage: async (): Promise<ShoeboxUsage[]> => {
    const response = await apiClient.get('/shoeboxes/usage');
    return response.data;
  },

  // Create a new shoebox
  createShoebox: async (name: string, description?: string): Promise<Shoebox> => {
    const response = await apiClient.post('/shoeboxes', { name, description });
    return response.data;
  },

  // Get a shoebox by ID
  getShoebox: async (id: string): Promise<Shoebox> => {
    const response = await apiClient.get(`/shoeboxes/${id}`);
    return response.data;
  },

  // Update a shoebox
  updateShoebox: async (id: string, name: string, description?: string): Promise<Shoebox> => {
    const response = await apiClient.put(`/shoeboxes/${id}`, { name, description });
    return response.data;
  },

  // Delete a shoebox
  deleteShoebox: async (id: string): Promise<void> => {
    await apiClient.delete(`/shoeboxes/${id}`);
  },

  // Add a video to a shoebox
  addVideoToShoebox: async (shoeboxId: string, videoId: string): Promise<void> => {
    await apiClient.put(`/shoeboxes/${shoeboxId}/videos/${videoId}`);
  },

  // Remove a video from a shoebox
  removeVideoFromShoebox: async (shoeboxId: string, videoId: string): Promise<void> => {
    await apiClient.delete(`/shoeboxes/${shoeboxId}/videos/${videoId}`);
  },

  // Get videos in a shoebox
  getVideosInShoebox: async (shoeboxId: string): Promise<string[]> => {
    const response = await apiClient.get(`/shoeboxes/${shoeboxId}/videos`);
    return response.data;
  },

  // Cleanup unused shoeboxes
  cleanupUnusedShoeboxes: async (): Promise<{ count: number }> => {
    const response = await apiClient.post('/shoeboxes/cleanup');
    return response.data;
  },
};

export default apiClient;
