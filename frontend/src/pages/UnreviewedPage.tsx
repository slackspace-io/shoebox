import React, { useState, useEffect, useMemo } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import {
  Box,
  Flex,
  Heading,
  Text,
  Button,
  VStack,
  useToast,
  Spinner,
  useColorModeValue,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  Code,
  useDisclosure,
  SimpleGrid
} from '@chakra-ui/react';
import { FaSave, FaArrowLeft, FaBug, FaChevronDown, FaChevronUp, FaArrowRight, FaCalendarAlt } from 'react-icons/fa';
import ReactPlayer from 'react-player';
import { videoApi, VideoWithMetadata, UpdateVideoDto, VideoSearchParams } from '../api/client';
import SearchFilters from '../components/SearchFilters';
import VideoForm from '../components/VideoForm';
import config from '../config';

interface SelectOption {
  value: string;
  label: string;
}

const UnreviewedPage: React.FC = () => {
  const navigate = useNavigate();
  const routerLocation = useLocation();
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const [videos, setVideos] = useState<VideoWithMetadata[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState<Record<string, boolean>>({});
  const [rawDatabaseValues, setRawDatabaseValues] = useState<string>('');
  const [searchParams, setSearchParams] = useState<VideoSearchParams>({
    unreviewed: true,
    limit: 100,
    offset: 0,
    sort_by: 'created_date',
    sort_order: 'ASC'
  });

  // State to track all available dates with videos
  const [allDates, setAllDates] = useState<string[]>([]);

  // State for video display
  const [visibleVideosCount, setVisibleVideosCount] = useState(config.unreviewed.defaultVideosToShow);
  const [expanded, setExpanded] = useState(false);

  // Form state for multiple videos
  const [formData, setFormData] = useState<Record<string, {
    title: string;
    description: string;
    rating?: number;
    location: string;
    event: string;
    selectedTags: SelectOption[];
    selectedPeople: SelectOption[];
    selectedShoeboxes: SelectOption[];
  }>>({});

  const borderColor = useColorModeValue('gray.200', 'gray.700');

  // Parse URL parameters and update search params
  useEffect(() => {
    const urlParams = new URLSearchParams(routerLocation.search);
    const start_date = urlParams.get('start_date');
    const end_date = urlParams.get('end_date');
    const tags = urlParams.get('tags');
    const people = urlParams.get('people');
    const rating = urlParams.get('rating');
    const sort_by = urlParams.get('sort_by');
    const sort_order = urlParams.get('sort_order');
    const min_duration = urlParams.get('min_duration');
    const max_duration = urlParams.get('max_duration');

    setSearchParams(prevParams => ({
      ...prevParams,
      start_date: start_date || undefined,
      end_date: end_date || undefined,
      tags: tags ? tags.split(',') : undefined,
      people: people ? people.split(',') : undefined,
      rating: rating ? parseInt(rating, 10) : undefined,
      sort_by: sort_by || prevParams.sort_by,
      sort_order: sort_order || prevParams.sort_order,
      min_duration: min_duration ? parseInt(min_duration, 10) : undefined,
      max_duration: max_duration ? parseInt(max_duration, 10) : undefined,
      unreviewed: true // Always keep unreviewed filter
    }));
  }, [routerLocation]);

  // Fetch all dates with unreviewed videos
  useEffect(() => {
    const fetchAllDates = async () => {
      try {
        // Create a search params object without date filters
        const allDatesParams: VideoSearchParams = {
          unreviewed: true,
          limit: 1000, // Use a large limit to get all videos
          offset: 0,
          sort_by: 'created_date',
          sort_order: 'ASC'
        };

        const allVideos = await videoApi.searchVideos(allDatesParams);

        // Extract unique dates from all videos
        const dates = new Set<string>();
        allVideos.forEach(video => {
          if (video.created_date) {
            const date = new Date(video.created_date).toISOString().split('T')[0];
            dates.add(date);
          }
        });

        // Convert Set to sorted array
        const sortedDates = Array.from(dates).sort();
        setAllDates(sortedDates);
      } catch (error) {
        console.error('Error fetching all dates:', error);
        toast({
          title: 'Error fetching date information',
          status: 'error',
          duration: 3000,
          isClosable: true,
        });
      }
    };

    fetchAllDates();
  }, [toast]); // Only run once when component mounts, include toast in dependencies

  // Load unreviewed videos
  useEffect(() => {
    const fetchUnreviewedVideos = async () => {
      setLoading(true);
      try {
        const results = await videoApi.searchVideos(searchParams);
        setVideos(results);

        if (results.length > 0) {
          // Initialize form data for all videos
          const newFormData: Record<string, any> = {};
          results.forEach(video => {
            newFormData[video.id] = initializeFormData(video);
          });
          setFormData(newFormData);
        }
      } catch (error) {
        console.error('Error fetching unreviewed videos:', error);
        toast({
          title: 'Error fetching unreviewed videos',
          status: 'error',
          duration: 3000,
          isClosable: true,
        });
      } finally {
        setLoading(false);
      }
    };

    fetchUnreviewedVideos();
  }, [searchParams, toast]);

  // Initialize form data for a video
  const initializeFormData = (video: VideoWithMetadata) => {
    return {
      title: video.title || '',
      description: video.description || '',
      rating: video.rating,
      location: video.location || '',
      event: video.event || '',
      selectedTags: video.tags.map(tag => ({ value: tag, label: tag })),
      selectedPeople: video.people.map(person => ({ value: person, label: person })),
      selectedShoeboxes: video.shoeboxes.map(shoebox => ({ value: shoebox, label: shoebox }))
    };
  };

  // Handle save for a specific video
  const handleSave = async (videoId: string) => {
    if (!formData[videoId]) return;

    setSaving(prev => ({ ...prev, [videoId]: true }));
    try {
      const videoFormData = formData[videoId];
      const updateData: UpdateVideoDto = {
        title: videoFormData.title || undefined,
        description: videoFormData.description || undefined,
        rating: videoFormData.rating,
        location: videoFormData.location || undefined,
        event: videoFormData.event || undefined,
        tags: videoFormData.selectedTags.map(tag => tag.value),
        people: videoFormData.selectedPeople.map(person => person.value),
        shoeboxes: videoFormData.selectedShoeboxes.map(shoebox => shoebox.value),
      };

      await videoApi.updateVideo(videoId, updateData);

      toast({
        title: 'Video updated',
        status: 'success',
        duration: 2000,
        isClosable: true,
      });

      // Remove the saved video from the videos array
      setVideos(prev => {
        // Create a new array without the saved video
        const newVideos = prev.filter(v => v.id !== videoId);

        // If we need to fetch more videos to maintain the same number of visible videos,
        // we can do that here in a separate effect
        if (newVideos.length < prev.length &&
            visibleVideosCount >= newVideos.length &&
            newVideos.length > 0) {
          // We'll trigger a fetch for more videos if needed
          fetchMoreVideosIfNeeded(newVideos.length);
        }

        return newVideos;
      });

      // Remove the form data for the saved video
      setFormData(prev => {
        const newFormData = { ...prev };
        delete newFormData[videoId];
        return newFormData;
      });
    } catch (error) {
      console.error('Error updating video:', error);
      toast({
        title: 'Error updating video',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    } finally {
      setSaving(prev => ({ ...prev, [videoId]: false }));
    }
  };

  // Handle showing debug information for a specific video
  const handleShowDebug = async (videoId: string) => {
    try {
      // Fetch the latest data from the server
      const videoData = await videoApi.getVideo(videoId);
      setRawDatabaseValues(JSON.stringify(videoData, null, 2));
      onOpen();
    } catch (error) {
      console.error('Error fetching video data:', error);
      toast({
        title: 'Error fetching video data',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    }
  };

  // Fetch more videos if needed to maintain the same number of visible videos
  const fetchMoreVideosIfNeeded = async (currentCount: number) => {
    // Calculate how many more videos we need
    const neededCount = visibleVideosCount - currentCount;

    if (neededCount <= 0) return; // No need to fetch more

    try {
      // Create a new search params object with an offset to get videos beyond the ones we already have
      const moreVideosParams: VideoSearchParams = {
        ...searchParams,
        offset: currentCount,
        limit: neededCount
      };

      const additionalVideos = await videoApi.searchVideos(moreVideosParams);

      if (additionalVideos.length > 0) {
        // Add the new videos to our state
        setVideos(prev => [...prev, ...additionalVideos]);

        // Initialize form data for the new videos
        const newFormData: Record<string, any> = {};
        additionalVideos.forEach(video => {
          newFormData[video.id] = initializeFormData(video);
        });

        // Update form data state
        setFormData(prev => ({
          ...prev,
          ...newFormData
        }));
      }
    } catch (error) {
      console.error('Error fetching additional videos:', error);
      toast({
        title: 'Error fetching additional videos',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    }
  };

  // Toggle expand/collapse for showing more/fewer videos
  const toggleExpand = () => {
    if (expanded) {
      setVisibleVideosCount(config.unreviewed.defaultVideosToShow);
      setExpanded(false);
    } else {
      setVisibleVideosCount(Math.min(videos.length, config.unreviewed.maxVideosToShow));
      setExpanded(true);
    }
  };



  // Get context information about videos from the same day, previous day, and next day
  const dateContext = useMemo(() => {
    if (videos.length === 0 || allDates.length === 0) return null;

    // Get the date of the first visible video
    const firstVisibleVideo = videos[0];
    if (!firstVisibleVideo.created_date) return null;

    const firstDate = new Date(firstVisibleVideo.created_date).toISOString().split('T')[0];
    const firstDateIndex = allDates.indexOf(firstDate);

    if (firstDateIndex === -1) return null;

    // Count videos for the current date
    const currentDateCount = videos.filter(video => {
      if (!video.created_date) return false;
      const videoDate = new Date(video.created_date).toISOString().split('T')[0];
      return videoDate === firstDate;
    }).length;

    const result = {
      currentDate: firstDate,
      currentDateCount,
      nextDate: null as string | null,
      nextDateCount: 0,
      previousDate: null as string | null,
      previousDateCount: 0
    };

    // Find the next date with videos
    if (firstDateIndex < allDates.length - 1) {
      result.nextDate = allDates[firstDateIndex + 1];
      // We don't know the exact count without fetching, but we know there's at least one
      result.nextDateCount = 1;
    }

    // Find the previous date with videos
    if (firstDateIndex > 0) {
      result.previousDate = allDates[firstDateIndex - 1];
      // We don't know the exact count without fetching, but we know there's at least one
      result.previousDateCount = 1;
    }

    return result;
  }, [videos, allDates]);

  // Handle jumping to next or previous day
  const handleJumpToDay = (targetDate: string | null) => {
    if (!targetDate) return;

    // Create a new filter with the target date as both start and end date
    const newFilters: Partial<VideoSearchParams> = {
      ...searchParams,
      start_date: targetDate,
      end_date: targetDate,
      offset: 0 // Reset pagination
    };

    // Apply the new filters
    handleFilterChange(newFilters);
  };

  // Handle filter changes
  const handleFilterChange = (newFilters: Partial<VideoSearchParams>) => {
    // Always keep unreviewed: true
    setSearchParams({
      ...searchParams,
      ...newFilters,
      unreviewed: true,
      offset: 0, // Reset pagination when filters change
    });

    // Update URL with filter parameters
    const urlParams = new URLSearchParams();

    if (newFilters.start_date) urlParams.set('start_date', newFilters.start_date);
    if (newFilters.end_date) urlParams.set('end_date', newFilters.end_date);
    if (newFilters.tags && newFilters.tags.length > 0) urlParams.set('tags', newFilters.tags.join(','));
    if (newFilters.people && newFilters.people.length > 0) urlParams.set('people', newFilters.people.join(','));
    if (newFilters.rating) urlParams.set('rating', newFilters.rating.toString());
    if (newFilters.sort_by) urlParams.set('sort_by', newFilters.sort_by);
    if (newFilters.sort_order) urlParams.set('sort_order', newFilters.sort_order);
    if (newFilters.min_duration) urlParams.set('min_duration', newFilters.min_duration.toString());
    if (newFilters.max_duration) urlParams.set('max_duration', newFilters.max_duration.toString());

    // Navigate to the same page with updated query parameters
    navigate({ pathname: routerLocation.pathname, search: urlParams.toString() });
  };


  if (loading) {
    return (
      <Flex justify="center" align="center" h="400px">
        <Spinner size="xl" />
      </Flex>
    );
  }

  if (videos.length === 0) {
    return (
      <Box textAlign="center" py={10}>
        <Heading>No unreviewed videos</Heading>
        <Text mt={4}>All videos have been reviewed.</Text>
        <Button mt={4} leftIcon={<FaArrowLeft />} onClick={() => navigate('/')}>
          Back to Videos
        </Button>
      </Box>
    );
  }

  // Get visible videos
  const visibleVideos = videos.slice(0, visibleVideosCount);

  return (
    <Box>
      <Flex mb={6} justify="space-between" align="center">
        <Button leftIcon={<FaArrowLeft />} onClick={() => navigate('/')}>
          Back to Videos
        </Button>
        <Text>
          Showing {visibleVideos.length} of {videos.length} unreviewed videos
        </Text>
      </Flex>

      {/* Date context information */}
      {dateContext && (
        <Box mb={4} p={4} borderWidth="1px" borderRadius="md" borderColor={borderColor}>
          <Heading size="md" mb={2}>Video Context</Heading>
          <Flex direction={{ base: 'column', md: 'row' }} gap={4} justify="space-between">
            {/* Previous Day */}
            <Box>
              <Flex align="center" mb={1}>
                <Text fontWeight="bold" mr={2}>
                  Previous Day: {dateContext.previousDate
                    ? new Date(dateContext.previousDate).toLocaleDateString()
                    : 'None'}
                </Text>
                <Button
                  size="sm"
                  leftIcon={<FaArrowLeft />}
                  isDisabled={!dateContext.previousDate}
                  onClick={() => handleJumpToDay(dateContext.previousDate)}
                  colorScheme="blue"
                  variant="outline"
                >
                  Jump
                </Button>
              </Flex>
              <Text>
                {dateContext.previousDate
                  ? `${dateContext.previousDateCount} videos from this day`
                  : 'No videos on any previous day'}
              </Text>
            </Box>

            {/* Current Day */}
            <Box>
              <Flex align="center" mb={1}>
                <Text fontWeight="bold" mr={2}>
                  Current Day: {new Date(dateContext.currentDate).toLocaleDateString()}
                </Text>
                <Button
                  size="sm"
                  leftIcon={<FaCalendarAlt />}
                  colorScheme="teal"
                  variant="outline"
                  isDisabled
                >
                  Current
                </Button>
              </Flex>
              <Text>
                {dateContext.currentDateCount} videos from this day
              </Text>
            </Box>

            {/* Next Day */}
            <Box>
              <Flex align="center" mb={1}>
                <Text fontWeight="bold" mr={2}>
                  Next Day: {dateContext.nextDate
                    ? new Date(dateContext.nextDate).toLocaleDateString()
                    : 'None'}
                </Text>
                <Button
                  size="sm"
                  rightIcon={<FaArrowRight />}
                  isDisabled={!dateContext.nextDate}
                  onClick={() => handleJumpToDay(dateContext.nextDate)}
                  colorScheme="blue"
                  variant="outline"
                >
                  Jump
                </Button>
              </Flex>
              <Text>
                {dateContext.nextDate
                  ? `${dateContext.nextDateCount} videos from this day`
                  : 'No videos on any future day'}
              </Text>
            </Box>
          </Flex>
        </Box>
      )}

      <SearchFilters onFilterChange={handleFilterChange} initialFilters={searchParams} />

      {/* Expand/Collapse button */}
      <Button
        onClick={toggleExpand}
        leftIcon={expanded ? <FaChevronUp /> : <FaChevronDown />}
        mb={4}
        mt={2}
      >
        {expanded ? 'Show Fewer Videos' : 'Show More Videos'}
      </Button>

      {/* Multiple videos grid */}
      <SimpleGrid columns={{ base: 1, xl: 2 }} spacing={8}>
        {visibleVideos.map(video => (
          <Box
            key={video.id}
            p={4}
            borderWidth="1px"
            borderRadius="md"
            borderColor={borderColor}
            mb={6}
          >
            <Flex direction={{ base: 'column', lg: 'row' }} gap={6}>
              <Box flex="1" maxW={{ lg: '50%' }}>
                <Box
                  borderRadius="md"
                  overflow="hidden"
                  borderWidth="1px"
                  borderColor={borderColor}
                  mb={4}
                >
                  <ReactPlayer
                    url={`/api/videos/${video.id}/stream`}
                    controls
                    width="100%"
                    height="auto"
                    style={{ aspectRatio: '16/9' }}
                  />
                </Box>

                <Box>
                  <Text fontSize="sm" color="gray.500">
                    File: {video.file_path}
                  </Text>
                  <Text fontSize="sm" color="gray.500">
                    Size: {video.file_size ? `${(video.file_size / (1024 * 1024)).toFixed(2)} MB` : 'Unknown'}
                  </Text>
                  {video.created_date && (
                    <Text fontSize="sm" color="gray.500">
                      Created: {new Date(video.created_date).toLocaleDateString()}
                    </Text>
                  )}
                  <Button
                    leftIcon={<FaBug />}
                    size="sm"
                    mt={2}
                    colorScheme="gray"
                    variant="outline"
                    onClick={() => handleShowDebug(video.id)}
                  >
                    Debug
                  </Button>
                </Box>
              </Box>

              <VStack align="stretch" flex="1" spacing={4}>
                {formData[video.id] && (
                  <VideoForm
                    video={video}
                    formData={formData[video.id]}
                    onChange={(updatedData) => {
                      setFormData(prev => {
                        const newFormData = { ...prev };
                        const videoFormData = { ...newFormData[video.id] };

                        // Map the properties from updatedData to the corresponding properties in the form data
                        if (updatedData.tags !== undefined) {
                          videoFormData.selectedTags = updatedData.tags.map(tag => ({ value: tag, label: tag }));
                        }
                        if (updatedData.people !== undefined) {
                          videoFormData.selectedPeople = updatedData.people.map(person => ({ value: person, label: person }));
                        }
                        if (updatedData.shoeboxes !== undefined) {
                          videoFormData.selectedShoeboxes = updatedData.shoeboxes.map(shoebox => ({ value: shoebox, label: shoebox }));
                        }

                        // Update other properties directly
                        if (updatedData.title !== undefined) videoFormData.title = updatedData.title;
                        if (updatedData.description !== undefined) videoFormData.description = updatedData.description;
                        if (updatedData.rating !== undefined) videoFormData.rating = updatedData.rating;
                        if (updatedData.location !== undefined) videoFormData.location = updatedData.location;
                        if (updatedData.event !== undefined) videoFormData.event = updatedData.event;

                        newFormData[video.id] = videoFormData;
                        return newFormData;
                      });
                    }}
                  />
                )}

                <Button
                  leftIcon={<FaSave />}
                  colorScheme="green"
                  onClick={() => handleSave(video.id)}
                  isLoading={saving[video.id]}
                  mt={4}
                >
                  Save
                </Button>
              </VStack>
            </Flex>
          </Box>
        ))}
      </SimpleGrid>

      {/* Debug Modal */}
      <Modal isOpen={isOpen} onClose={onClose} size="xl">
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Raw Database Values</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <Box overflowX="auto">
              <Code display="block" whiteSpace="pre" p={4} borderRadius="md">
                {rawDatabaseValues}
              </Code>
            </Box>
          </ModalBody>
          <ModalFooter>
            <Button colorScheme="blue" mr={3} onClick={onClose}>
              Close
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </Box>
  );
};

export default UnreviewedPage;
