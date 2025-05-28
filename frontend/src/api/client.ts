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
  created_at: string;
  updated_at: string;
}

export interface VideoWithMetadata extends Video {
  tags: string[];
  people: string[];
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
  tags: string[];
  people: string[];
}

export interface UpdateVideoDto {
  title?: string;
  description?: string;
  rating?: number;
  tags?: string[];
  people?: string[];
}

export interface VideoSearchParams {
  query?: string;
  tags?: string[];
  people?: string[];
  rating?: number;
  limit?: number;
  offset?: number;
  unreviewed?: boolean;
  sort_by?: string;
  sort_order?: string;
  start_date?: string;
  end_date?: string;
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

export default apiClient;
