import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import {
  Box,
  Flex,
  Heading,
  Text,
  Button,
  FormControl,
  FormLabel,
  Input,
  Textarea,
  HStack,
  VStack,
  IconButton,
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
  useDisclosure
} from '@chakra-ui/react';
import { FaSave, FaArrowLeft, FaArrowRight, FaStar, FaRegStar, FaBug } from 'react-icons/fa';
import ReactPlayer from 'react-player';
import CreatableSelect from 'react-select/creatable';
import { videoApi, tagApi, personApi, VideoWithMetadata, UpdateVideoDto, VideoSearchParams } from '../api/client';
import SearchFilters from '../components/SearchFilters';

interface SelectOption {
  value: string;
  label: string;
}

const UnreviewedPage: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const [videos, setVideos] = useState<VideoWithMetadata[]>([]);
  const [currentVideoIndex, setCurrentVideoIndex] = useState(0);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [loadingNextVideo, setLoadingNextVideo] = useState(false);
  const [rawDatabaseValues, setRawDatabaseValues] = useState<string>('');
  const [searchParams, setSearchParams] = useState<VideoSearchParams>({
    unreviewed: true,
    limit: 100,
    offset: 0,
    sort_by: 'created_date',
    sort_order: 'ASC'
  });

  // Form state
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [rating, setRating] = useState<number | undefined>(undefined);
  const [selectedTags, setSelectedTags] = useState<SelectOption[]>([]);
  const [selectedPeople, setSelectedPeople] = useState<SelectOption[]>([]);

  // Options for select inputs
  const [tagOptions, setTagOptions] = useState<SelectOption[]>([]);
  const [peopleOptions, setPeopleOptions] = useState<SelectOption[]>([]);

  const bgColor = useColorModeValue('white', 'gray.800');
  const borderColor = useColorModeValue('gray.200', 'gray.700');

  // Parse URL parameters and update search params
  useEffect(() => {
    const urlParams = new URLSearchParams(location.search);
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
  }, [location]);

  // Load unreviewed videos
  useEffect(() => {
    const fetchUnreviewedVideos = async () => {
      setLoading(true);
      try {
        const results = await videoApi.searchVideos(searchParams);
        setVideos(results);

        if (results.length > 0) {
          // Initialize form with the first video
          initializeForm(results[0]);
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

  // Load tags and people options
  useEffect(() => {
    const fetchOptions = async () => {
      try {
        // Fetch tags
        const tags = await tagApi.getTags();
        setTagOptions(tags.map(tag => ({ value: tag.name, label: tag.name })));

        // Fetch people
        const people = await personApi.getPeople();
        setPeopleOptions(people.map(person => ({ value: person.name, label: person.name })));
      } catch (error) {
        console.error('Error fetching options:', error);
      }
    };

    fetchOptions();
  }, []);

  // Initialize form with video data
  const initializeForm = (video: VideoWithMetadata) => {
    setTitle(video.title || '');
    setDescription(video.description || '');
    setRating(video.rating);
    setSelectedTags(video.tags.map(tag => ({ value: tag, label: tag })));
    setSelectedPeople(video.people.map(person => ({ value: person, label: person })));
  };

  // Handle save and move to next video
  const handleSaveAndNext = async () => {
    if (videos.length === 0 || currentVideoIndex >= videos.length) return;

    const currentVideo = videos[currentVideoIndex];
    setSaving(true);
    try {
      const updateData: UpdateVideoDto = {
        title: title || undefined,
        description: description || undefined,
        rating,
        tags: selectedTags.map(tag => tag.value),
        people: selectedPeople.map(person => person.value),
      };

      await videoApi.updateVideo(currentVideo.id, updateData);

      toast({
        title: 'Video updated',
        status: 'success',
        duration: 2000,
        isClosable: true,
      });

      // Move to next video if available
      if (currentVideoIndex < videos.length - 1) {
        setLoadingNextVideo(true);
        setCurrentVideoIndex(currentVideoIndex + 1);
        initializeForm(videos[currentVideoIndex + 1]);
        setLoadingNextVideo(false);
      } else {
        // If this was the last video, refresh the list to get more unreviewed videos
        setLoadingNextVideo(true);
        // Keep the current search parameters but reset offset
        const nextSearchParams = {
          ...searchParams,
          offset: 0
        };
        const results = await videoApi.searchVideos(nextSearchParams);

        if (results.length > 0) {
          setVideos(results);
          setCurrentVideoIndex(0);
          initializeForm(results[0]);
        } else {
          // No more unreviewed videos
          toast({
            title: 'All videos reviewed',
            description: 'There are no more unreviewed videos.',
            status: 'info',
            duration: 3000,
            isClosable: true,
          });
          navigate('/');
        }
        setLoadingNextVideo(false);
      }
    } catch (error) {
      console.error('Error updating video:', error);
      toast({
        title: 'Error updating video',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    } finally {
      setSaving(false);
    }
  };

  // Handle rating change
  const handleRatingChange = (newRating: number) => {
    setRating(newRating === rating ? undefined : newRating);
  };

  // Handle showing debug information
  const handleShowDebug = async () => {
    if (videos.length === 0 || currentVideoIndex >= videos.length) return;

    const currentVideo = videos[currentVideoIndex];
    try {
      // Fetch the latest data from the server
      const videoData = await videoApi.getVideo(currentVideo.id);
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
    navigate({ pathname: location.pathname, search: urlParams.toString() });
  };

  // Render rating stars
  const renderRatingStars = () => {
    const stars = [];
    for (let i = 1; i <= 5; i++) {
      stars.push(
        <IconButton
          key={i}
          icon={i <= (rating || 0) ? <FaStar /> : <FaRegStar />}
          aria-label={`${i} star`}
          variant="ghost"
          color={i <= (rating || 0) ? 'yellow.400' : 'gray.400'}
          onClick={() => handleRatingChange(i)}
        />
      );
    }
    return <HStack spacing={1}>{stars}</HStack>;
  };

  // Custom styles for react-select
  const selectStyles = {
    control: (base: any) => ({
      ...base,
      background: bgColor,
      borderColor: borderColor,
    }),
    menu: (base: any) => ({
      ...base,
      background: bgColor,
      zIndex: 2
    }),
    option: (base: any, state: any) => ({
      ...base,
      backgroundColor: state.isFocused
        ? useColorModeValue('blue.50', 'blue.900')
        : useColorModeValue('white', 'gray.700'),
      color: useColorModeValue('black', 'black')
    })
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

  const currentVideo = videos[currentVideoIndex];

  return (
    <Box>
      <Flex mb={6} justify="space-between" align="center">
        <Button leftIcon={<FaArrowLeft />} onClick={() => navigate('/')}>
          Back to Videos
        </Button>
        <Text>
          Reviewing {currentVideoIndex + 1} of {videos.length}
        </Text>
      </Flex>

      <SearchFilters onFilterChange={handleFilterChange} initialFilters={searchParams} />

      <Flex direction={{ base: 'column', lg: 'row' }} gap={8}>
        <Box flex="1" maxW={{ lg: '60%' }}>
          <Box
            borderRadius="md"
            overflow="hidden"
            borderWidth="1px"
            borderColor={borderColor}
          >
            {loadingNextVideo ? (
              <Flex justify="center" align="center" h="300px">
                <Spinner size="xl" />
              </Flex>
            ) : (
              <ReactPlayer
                url={`/api/videos/${currentVideo.id}/stream`}
                controls
                width="100%"
                height="auto"
                style={{ aspectRatio: '16/9' }}
              />
            )}
          </Box>

          <Box mt={4}>
            <Text fontSize="sm" color="gray.500">
              File: {currentVideo.file_path}
            </Text>
            <Text fontSize="sm" color="gray.500">
              Size: {currentVideo.file_size ? `${(currentVideo.file_size / (1024 * 1024)).toFixed(2)} MB` : 'Unknown'}
            </Text>
            {currentVideo.created_date && (
              <Text fontSize="sm" color="gray.500">
                Created: {new Date(currentVideo.created_date).toLocaleDateString()}
              </Text>
            )}
            <Button
              leftIcon={<FaBug />}
              size="sm"
              mt={2}
              colorScheme="gray"
              variant="outline"
              onClick={handleShowDebug}
            >
              Debug
            </Button>
          </Box>
        </Box>

        <VStack align="stretch" flex="1" spacing={4}>
          <FormControl>
            <FormLabel>Title</FormLabel>
            <Input
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="Enter title"
            />
          </FormControl>

          <FormControl>
            <FormLabel>Rating</FormLabel>
            {renderRatingStars()}
          </FormControl>

          <FormControl>
            <FormLabel>Description</FormLabel>
            <Textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Enter description"
              rows={4}
            />
          </FormControl>

          <FormControl>
            <FormLabel>Tags</FormLabel>
            <CreatableSelect
              isMulti
              options={tagOptions}
              value={selectedTags}
              onChange={(selected: any) => setSelectedTags(selected || [])}
              placeholder="Select or create tags..."
              styles={selectStyles}
              isClearable
              formatCreateLabel={(inputValue) => `Create tag "${inputValue}"`}
              onCreateOption={async (inputValue) => {
                try {
                  const newTag = await tagApi.createTag(inputValue);
                  const newOption = { value: newTag.name, label: newTag.name };
                  setTagOptions([...tagOptions, newOption]);
                  setSelectedTags([...selectedTags, newOption]);
                  toast({
                    title: 'Tag created',
                    status: 'success',
                    duration: 2000,
                    isClosable: true,
                  });
                } catch (error) {
                  console.error('Error creating tag:', error);
                  toast({
                    title: 'Error creating tag',
                    status: 'error',
                    duration: 3000,
                    isClosable: true,
                  });
                }
              }}
            />
          </FormControl>

          <FormControl>
            <FormLabel>People</FormLabel>
            <CreatableSelect
              isMulti
              options={peopleOptions}
              value={selectedPeople}
              onChange={(selected: any) => setSelectedPeople(selected || [])}
              placeholder="Select or create people..."
              styles={selectStyles}
              isClearable
              formatCreateLabel={(inputValue) => `Create person "${inputValue}"`}
              onCreateOption={async (inputValue) => {
                try {
                  const newPerson = await personApi.createPerson(inputValue);
                  const newOption = { value: newPerson.name, label: newPerson.name };
                  setPeopleOptions([...peopleOptions, newOption]);
                  setSelectedPeople([...selectedPeople, newOption]);
                  toast({
                    title: 'Person created',
                    status: 'success',
                    duration: 2000,
                    isClosable: true,
                  });
                } catch (error) {
                  console.error('Error creating person:', error);
                  toast({
                    title: 'Error creating person',
                    status: 'error',
                    duration: 3000,
                    isClosable: true,
                  });
                }
              }}
            />
          </FormControl>

          <Button
            leftIcon={<FaSave />}
            rightIcon={<FaArrowRight />}
            colorScheme="green"
            onClick={handleSaveAndNext}
            isLoading={saving || loadingNextVideo}
            mt={4}
            size="lg"
          >
            Save & Next
          </Button>
        </VStack>
      </Flex>

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
