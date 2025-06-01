import React, { useState, useEffect } from 'react';
import {
  Box,
  Heading,
  Text,
  Button,
  Flex,
  VStack,
  Checkbox,
  Spinner,
  useToast,
  Alert,
  AlertIcon,
  AlertTitle,
  AlertDescription,
  useColorModeValue,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  Badge,
  Image,
  InputGroup,
  InputRightElement,
  IconButton,
  Input,
  FormControl,
  FormLabel,
  Select,
  Tag,
  TagLabel,
  TagCloseButton,
} from '@chakra-ui/react';
import { FaSearch, FaEdit, FaStar } from 'react-icons/fa';
import { videoApi, tagApi, personApi, locationApi, eventApi, VideoWithMetadata, UpdateVideoDto, BulkUpdateVideoDto } from '../api/client';
import SearchFilters from '../components/SearchFilters';

const BulkEditPage: React.FC = () => {
  const [videos, setVideos] = useState<VideoWithMetadata[]>([]);
  const [selectedVideos, setSelectedVideos] = useState<string[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(true);
  const [updating, setUpdating] = useState(false);
  const [availableTags, setAvailableTags] = useState<string[]>([]);
  const [availablePeople, setAvailablePeople] = useState<string[]>([]);
  const [availableLocations, setAvailableLocations] = useState<string[]>([]);
  const [availableEvents, setAvailableEvents] = useState<string[]>([]);
  const [newTag, setNewTag] = useState('');
  const [newPerson, setNewPerson] = useState('');
  const [newLocation, setNewLocation] = useState('');
  const [newEvent, setNewEvent] = useState('');
  const [selectedTags, setSelectedTags] = useState<string[]>([]);
  const [selectedPeople, setSelectedPeople] = useState<string[]>([]);
  const [selectedLocation, setSelectedLocation] = useState<string | null>(null);
  const [selectedEvent, setSelectedEvent] = useState<string | null>(null);
  const [selectedRating, setSelectedRating] = useState<number | null>(null);
  const toast = useToast();

  const bgColor = useColorModeValue('white', 'gray.800');

  // Load videos and metadata on component mount
  useEffect(() => {
    fetchVideos();
    fetchTags();
    fetchPeople();
    fetchLocations();
    fetchEvents();
  }, []);

  // Fetch videos from API
  const fetchVideos = async (params = {}) => {
    setLoading(true);
    try {
      const results = await videoApi.searchVideos({
        limit: 100,
        ...params
      });
      setVideos(results);
    } catch (error) {
      console.error('Error fetching videos:', error);
      toast({
        title: 'Error fetching videos',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    } finally {
      setLoading(false);
    }
  };

  // Fetch available tags
  const fetchTags = async () => {
    try {
      const tags = await tagApi.getTagUsage();
      setAvailableTags(tags.map(tag => tag.name));
    } catch (error) {
      console.error('Error fetching tags:', error);
    }
  };

  // Fetch available people
  const fetchPeople = async () => {
    try {
      const people = await personApi.getPersonUsage();
      setAvailablePeople(people.map(person => person.name));
    } catch (error) {
      console.error('Error fetching people:', error);
    }
  };

  // Fetch available locations
  const fetchLocations = async () => {
    try {
      const locations = await locationApi.getLocations();
      setAvailableLocations(locations);
    } catch (error) {
      console.error('Error fetching locations:', error);
    }
  };

  // Fetch available events
  const fetchEvents = async () => {
    try {
      const events = await eventApi.getEvents();
      setAvailableEvents(events);
    } catch (error) {
      console.error('Error fetching events:', error);
    }
  };

  // Handle search input change
  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value);
  };

  // Handle search submit
  const handleSearch = () => {
    fetchVideos({
      query: searchQuery.trim() || undefined,
    });
  };

  // Handle search on Enter key
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  // Handle filter changes
  const handleFilterChange = (filters: any) => {
    fetchVideos({
      ...filters,
      query: searchQuery.trim() || undefined,
    });
  };

  // Toggle video selection
  const toggleVideoSelection = (videoId: string) => {
    setSelectedVideos(prev =>
      prev.includes(videoId)
        ? prev.filter(id => id !== videoId)
        : [...prev, videoId]
    );
  };

  // Select all videos
  const selectAllVideos = () => {
    if (selectedVideos.length === videos.length) {
      setSelectedVideos([]);
    } else {
      setSelectedVideos(videos.map(video => video.id));
    }
  };

  // Add a tag to the selection
  const addTag = (tag: string) => {
    if (tag && !selectedTags.includes(tag)) {
      setSelectedTags([...selectedTags, tag]);
      setNewTag('');
    }
  };

  // Remove a tag from the selection
  const removeTag = (tag: string) => {
    setSelectedTags(selectedTags.filter(t => t !== tag));
  };

  // Add a person to the selection
  const addPerson = (person: string) => {
    if (person && !selectedPeople.includes(person)) {
      setSelectedPeople([...selectedPeople, person]);
      setNewPerson('');
    }
  };

  // Remove a person from the selection
  const removePerson = (person: string) => {
    setSelectedPeople(selectedPeople.filter(p => p !== person));
  };

  // Set the selected location
  const setLocation = (location: string) => {
    if (location) {
      setSelectedLocation(location);
      setNewLocation('');
    }
  };

  // Clear the selected location
  const clearLocation = () => {
    setSelectedLocation(null);
  };

  // Set the selected event
  const setEvent = (event: string) => {
    if (event) {
      setSelectedEvent(event);
      setNewEvent('');
    }
  };

  // Clear the selected event
  const clearEvent = () => {
    setSelectedEvent(null);
  };

  // Handle bulk update
  const handleBulkUpdate = async () => {
    if (selectedVideos.length === 0) {
      toast({
        title: 'No videos selected',
        description: 'Please select at least one video to update',
        status: 'warning',
        duration: 3000,
        isClosable: true,
      });
      return;
    }

    if (!selectedRating && selectedTags.length === 0 && selectedPeople.length === 0 && !selectedLocation && !selectedEvent) {
      toast({
        title: 'No changes to apply',
        description: 'Please select a rating, tags, people, location, or event to update',
        status: 'warning',
        duration: 3000,
        isClosable: true,
      });
      return;
    }

    const updateDto: UpdateVideoDto = {};

    if (selectedRating !== null) {
      updateDto.rating = selectedRating;
    }

    if (selectedTags.length > 0) {
      updateDto.tags = selectedTags;
    }

    if (selectedPeople.length > 0) {
      updateDto.people = selectedPeople;
    }

    if (selectedLocation) {
      updateDto.location = selectedLocation;
    }

    if (selectedEvent) {
      updateDto.event = selectedEvent;
    }

    setUpdating(true);
    try {
      const bulkUpdateDto: BulkUpdateVideoDto = {
        video_ids: selectedVideos,
        update: updateDto
      };

      await videoApi.bulkUpdateVideos(bulkUpdateDto);

      toast({
        title: 'Videos updated',
        description: `Successfully updated ${selectedVideos.length} videos`,
        status: 'success',
        duration: 3000,
        isClosable: true,
      });

      // Refresh the videos list
      fetchVideos();

      // Reset selection
      setSelectedVideos([]);
      setSelectedRating(null);
      setSelectedTags([]);
      setSelectedPeople([]);
      setSelectedLocation(null);
      setSelectedEvent(null);

    } catch (error) {
      console.error('Error updating videos:', error);
      toast({
        title: 'Error updating videos',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    } finally {
      setUpdating(false);
    }
  };

  return (
    <Box>
      <Heading size="xl" mb={6}>Bulk Edit Videos</Heading>

      <VStack spacing={6} align="stretch">
        <Box p={4} borderWidth="1px" borderRadius="lg" bg={bgColor} borderColor="gray.200">
          <Heading size="md" mb={4}>1. Select Videos</Heading>

          <Flex mb={6} direction={{ base: 'column', md: 'row' }} gap={4}>
            <InputGroup size="md" flex="1">
              <Input
                placeholder="Search videos..."
                value={searchQuery}
                onChange={handleSearchChange}
                onKeyDown={handleKeyDown}
              />
              <InputRightElement>
                <IconButton
                  aria-label="Search"
                  icon={<FaSearch />}
                  size="sm"
                  onClick={handleSearch}
                />
              </InputRightElement>
            </InputGroup>
          </Flex>

          <SearchFilters onFilterChange={handleFilterChange} />

          {loading ? (
            <Flex justify="center" align="center" h="200px">
              <Spinner size="xl" />
            </Flex>
          ) : videos.length === 0 ? (
            <Alert status="info">
              <AlertIcon />
              <AlertTitle>No videos found</AlertTitle>
              <AlertDescription>Try adjusting your search criteria</AlertDescription>
            </Alert>
          ) : (
            <Box overflowX="auto">
              <Table variant="simple">
                <Thead>
                  <Tr>
                    <Th width="50px">
                      <Checkbox
                        isChecked={selectedVideos.length === videos.length && videos.length > 0}
                        isIndeterminate={selectedVideos.length > 0 && selectedVideos.length < videos.length}
                        onChange={selectAllVideos}
                      />
                    </Th>
                    <Th>Thumbnail</Th>
                    <Th>Title</Th>
                    <Th>Tags</Th>
                    <Th>People</Th>
                    <Th>Rating</Th>
                  </Tr>
                </Thead>
                <Tbody>
                  {videos.map(video => (
                    <Tr
                      key={video.id}
                      _hover={{ bg: useColorModeValue('gray.50', 'gray.700') }}
                      cursor="pointer"
                      onClick={() => toggleVideoSelection(video.id)}
                    >
                      <Td>
                        <Checkbox
                          isChecked={selectedVideos.includes(video.id)}
                          onChange={(e) => {
                            e.stopPropagation();
                            toggleVideoSelection(video.id);
                          }}
                        />
                      </Td>
                      <Td>
                        <Image
                          src={video.thumbnail_path || '/placeholder-thumbnail.jpg'}
                          alt={video.title || video.file_name}
                          boxSize="60px"
                          objectFit="cover"
                          borderRadius="md"
                          fallbackSrc="https://via.placeholder.com/60?text=No+Thumbnail"
                        />
                      </Td>
                      <Td>
                        <Text fontWeight="bold" noOfLines={1}>
                          {video.title || video.file_name}
                        </Text>
                        <Text fontSize="sm" color="gray.500" noOfLines={1}>
                          {video.duration ? `${Math.floor(video.duration / 60)}:${(video.duration % 60).toString().padStart(2, '0')}` : 'Unknown duration'}
                        </Text>
                      </Td>
                      <Td>
                        <Flex wrap="wrap" gap={1}>
                          {video.tags.slice(0, 3).map(tag => (
                            <Badge key={tag} colorScheme="blue" fontSize="xs" color="white">
                              {tag}
                            </Badge>
                          ))}
                          {video.tags.length > 3 && (
                            <Badge colorScheme="gray" fontSize="xs" color="white">
                              +{video.tags.length - 3}
                            </Badge>
                          )}
                        </Flex>
                      </Td>
                      <Td>
                        <Flex wrap="wrap" gap={1}>
                          {video.people.slice(0, 2).map(person => (
                            <Badge key={person} colorScheme="green" fontSize="xs" color="white">
                              {person}
                            </Badge>
                          ))}
                          {video.people.length > 2 && (
                            <Badge colorScheme="gray" fontSize="xs" color="white">
                              +{video.people.length - 2}
                            </Badge>
                          )}
                        </Flex>
                      </Td>
                      <Td>
                        {video.rating ? (
                          <Flex>
                            {[...Array(video.rating)].map((_, i) => (
                              <FaStar key={i} color="gold" />
                            ))}
                          </Flex>
                        ) : (
                          <Text fontSize="sm" color="gray.500">No rating</Text>
                        )}
                      </Td>
                    </Tr>
                  ))}
                </Tbody>
              </Table>
            </Box>
          )}

          <Flex justify="space-between" mt={4}>
            <Text>
              {selectedVideos.length} of {videos.length} videos selected
            </Text>
          </Flex>
        </Box>

        <Box p={4} borderWidth="1px" borderRadius="lg" bg={bgColor} borderColor="gray.200">
          <Heading size="md" mb={4}>2. Edit Selected Videos</Heading>

          <VStack spacing={4} align="stretch">
            <FormControl>
              <FormLabel>Rating</FormLabel>
              <Select
                placeholder="Select rating"
                value={selectedRating?.toString() || ''}
                onChange={(e) => setSelectedRating(e.target.value ? parseInt(e.target.value) : null)}
              >
                <option value="1">1 Star</option>
                <option value="2">2 Stars</option>
                <option value="3">3 Stars</option>
                <option value="4">4 Stars</option>
                <option value="5">5 Stars</option>
              </Select>
            </FormControl>

            <FormControl>
              <FormLabel>Tags</FormLabel>
              <Flex mb={2} wrap="wrap" gap={2}>
                {selectedTags.map(tag => (
                  <Tag key={tag} size="md" borderRadius="full" variant="solid" colorScheme="blue">
                    <TagLabel>{tag}</TagLabel>
                    <TagCloseButton onClick={() => removeTag(tag)} />
                  </Tag>
                ))}
              </Flex>
              <Flex>
                <Input
                  placeholder="Add a tag"
                  value={newTag}
                  onChange={(e) => setNewTag(e.target.value)}
                  list="available-tags"
                />
                <Button ml={2} onClick={() => addTag(newTag)}>Add</Button>
              </Flex>
              <datalist id="available-tags">
                {availableTags.map(tag => (
                  <option key={tag} value={tag} />
                ))}
              </datalist>
            </FormControl>

            <FormControl>
              <FormLabel>People</FormLabel>
              <Flex mb={2} wrap="wrap" gap={2}>
                {selectedPeople.map(person => (
                  <Tag key={person} size="md" borderRadius="full" variant="solid" colorScheme="green">
                    <TagLabel>{person}</TagLabel>
                    <TagCloseButton onClick={() => removePerson(person)} />
                  </Tag>
                ))}
              </Flex>
              <Flex>
                <Input
                  placeholder="Add a person"
                  value={newPerson}
                  onChange={(e) => setNewPerson(e.target.value)}
                  list="available-people"
                />
                <Button ml={2} onClick={() => addPerson(newPerson)}>Add</Button>
              </Flex>
              <datalist id="available-people">
                {availablePeople.map(person => (
                  <option key={person} value={person} />
                ))}
              </datalist>
            </FormControl>

            <FormControl>
              <FormLabel>Location</FormLabel>
              {selectedLocation ? (
                <Flex mb={2} wrap="wrap" gap={2}>
                  <Tag size="md" borderRadius="full" variant="solid" colorScheme="purple">
                    <TagLabel>{selectedLocation}</TagLabel>
                    <TagCloseButton onClick={clearLocation} />
                  </Tag>
                </Flex>
              ) : null}
              <Flex>
                <Input
                  placeholder="Set location"
                  value={newLocation}
                  onChange={(e) => setNewLocation(e.target.value)}
                  list="available-locations"
                  isDisabled={selectedLocation !== null}
                />
                <Button
                  ml={2}
                  onClick={() => setLocation(newLocation)}
                  isDisabled={selectedLocation !== null}
                >
                  Set
                </Button>
              </Flex>
              <datalist id="available-locations">
                {availableLocations.map(location => (
                  <option key={location} value={location} />
                ))}
              </datalist>
            </FormControl>

            <FormControl>
              <FormLabel>Event</FormLabel>
              {selectedEvent ? (
                <Flex mb={2} wrap="wrap" gap={2}>
                  <Tag size="md" borderRadius="full" variant="solid" colorScheme="orange">
                    <TagLabel>{selectedEvent}</TagLabel>
                    <TagCloseButton onClick={clearEvent} />
                  </Tag>
                </Flex>
              ) : null}
              <Flex>
                <Input
                  placeholder="Set event"
                  value={newEvent}
                  onChange={(e) => setNewEvent(e.target.value)}
                  list="available-events"
                  isDisabled={selectedEvent !== null}
                />
                <Button
                  ml={2}
                  onClick={() => setEvent(newEvent)}
                  isDisabled={selectedEvent !== null}
                >
                  Set
                </Button>
              </Flex>
              <datalist id="available-events">
                {availableEvents.map(event => (
                  <option key={event} value={event} />
                ))}
              </datalist>
            </FormControl>
          </VStack>

          <Button
            mt={6}
            colorScheme="blue"
            leftIcon={<FaEdit />}
            isLoading={updating}
            loadingText="Updating"
            onClick={handleBulkUpdate}
            isDisabled={selectedVideos.length === 0 ||
                       (selectedRating === null && selectedTags.length === 0 && selectedPeople.length === 0 && selectedLocation === null && selectedEvent === null)}
          >
            Update Selected Videos
          </Button>
        </Box>
      </VStack>
    </Box>
  );
};

export default BulkEditPage;
