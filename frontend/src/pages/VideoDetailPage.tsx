import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
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
  Badge,
  useDisclosure,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  useColorModeValue
} from '@chakra-ui/react';
import { FaEdit, FaSave, FaTrash, FaArrowLeft, FaStar, FaRegStar } from 'react-icons/fa';
import ReactPlayer from 'react-player';
import CreatableSelect from 'react-select/creatable';
import { videoApi, tagApi, personApi, VideoWithMetadata, UpdateVideoDto } from '../api/client';

interface SelectOption {
  value: string;
  label: string;
}

const VideoDetailPage: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const [video, setVideo] = useState<VideoWithMetadata | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [isEditing, setIsEditing] = useState(false);

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

  // Load video data
  useEffect(() => {
    const fetchVideo = async () => {
      if (!id) return;

      setLoading(true);
      try {
        const videoData = await videoApi.getVideo(id);
        setVideo(videoData);

        // Initialize form state
        setTitle(videoData.title || '');
        setDescription(videoData.description || '');
        setRating(videoData.rating);
        setSelectedTags(videoData.tags.map(tag => ({ value: tag, label: tag })));
        setSelectedPeople(videoData.people.map(person => ({ value: person, label: person })));
      } catch (error) {
        console.error('Error fetching video:', error);
        toast({
          title: 'Error fetching video',
          status: 'error',
          duration: 3000,
          isClosable: true,
        });
        navigate('/');
      } finally {
        setLoading(false);
      }
    };

    fetchVideo();
  }, [id, navigate, toast]);

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

  // Handle save
  const handleSave = async () => {
    if (!id || !video) return;

    setSaving(true);
    try {
      const updateData: UpdateVideoDto = {
        title: title || undefined,
        description: description || undefined,
        rating,
        tags: selectedTags.map(tag => tag.value),
        people: selectedPeople.map(person => person.value),
      };

      const updatedVideo = await videoApi.updateVideo(id, updateData);
      setVideo(updatedVideo);
      setIsEditing(false);

      toast({
        title: 'Video updated',
        status: 'success',
        duration: 3000,
        isClosable: true,
      });

      // Navigate back to the video detail page to ensure proper rendering
      navigate(`/videos/${id}`);
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

  // Handle delete
  const handleDelete = async () => {
    if (!id) return;

    setDeleting(true);
    try {
      await videoApi.deleteVideo(id);
      toast({
        title: 'Video deleted',
        status: 'success',
        duration: 3000,
        isClosable: true,
      });
      navigate('/');
    } catch (error) {
      console.error('Error deleting video:', error);
      toast({
        title: 'Error deleting video',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
      setDeleting(false);
    }
  };

  // Toggle edit mode
  const toggleEditMode = () => {
    if (isEditing) {
      // Reset form state when canceling edit
      if (video) {
        setTitle(video.title || '');
        setDescription(video.description || '');
        setRating(video.rating);
        setSelectedTags(video.tags.map(tag => ({ value: tag, label: tag })));
        setSelectedPeople(video.people.map(person => ({ value: person, label: person })));
      }
    }
    setIsEditing(!isEditing);
  };

  // Handle rating change
  const handleRatingChange = (newRating: number) => {
    setRating(newRating === rating ? undefined : newRating);
  };

  // Render rating stars
  const renderRatingStars = (editable = false) => {
    const stars = [];
    for (let i = 1; i <= 5; i++) {
      stars.push(
        <IconButton
          key={i}
          icon={i <= (rating || 0) ? <FaStar /> : <FaRegStar />}
          aria-label={`${i} star`}
          variant="ghost"
          color={i <= (rating || 0) ? 'yellow.400' : 'gray.400'}
          isDisabled={!editable}
          onClick={() => editable && handleRatingChange(i)}
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

  if (!video) {
    return (
      <Box textAlign="center" py={10}>
        <Heading>Video not found</Heading>
        <Button mt={4} leftIcon={<FaArrowLeft />} onClick={() => navigate('/')}>
          Back to Videos
        </Button>
      </Box>
    );
  }

  return (
    <Box>
      <Flex mb={6} justify="space-between" align="center">
        <Button leftIcon={<FaArrowLeft />} onClick={() => navigate('/')}>
          Back to Videos
        </Button>
        <HStack>
          <Button
            leftIcon={isEditing ? <FaSave /> : <FaEdit />}
            colorScheme={isEditing ? 'green' : 'blue'}
            onClick={isEditing ? handleSave : toggleEditMode}
            isLoading={saving}
          >
            {isEditing ? 'Save' : 'Edit'}
          </Button>
          {isEditing && (
            <Button variant="outline" onClick={toggleEditMode}>
              Cancel
            </Button>
          )}
          <Button
            leftIcon={<FaTrash />}
            colorScheme="red"
            onClick={onOpen}
          >
            Delete
          </Button>
        </HStack>
      </Flex>

      <Flex direction={{ base: 'column', lg: 'row' }} gap={8}>
        <Box flex="1" maxW={{ lg: '60%' }}>
          <Box
            borderRadius="md"
            overflow="hidden"
            borderWidth="1px"
            borderColor={borderColor}
          >
            <ReactPlayer
              url={`http://localhost:3000/api/videos/${video.id}/stream`}
              controls
              width="100%"
              height="auto"
              style={{ aspectRatio: '16/9' }}
            />
          </Box>

          <Box mt={4}>
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
          </Box>
        </Box>

        <VStack align="stretch" flex="1" spacing={4}>
          {isEditing ? (
            <FormControl>
              <FormLabel>Title</FormLabel>
              <Input
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="Enter title"
              />
            </FormControl>
          ) : (
            <Heading size="xl">{video.title || video.file_name}</Heading>
          )}

          <FormControl>
            <FormLabel>Rating</FormLabel>
            {renderRatingStars(isEditing)}
          </FormControl>

          <FormControl>
            <FormLabel>Description</FormLabel>
            {isEditing ? (
              <Textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Enter description"
                rows={4}
              />
            ) : (
              <Text>{video.description || 'No description'}</Text>
            )}
          </FormControl>

          <FormControl>
            <FormLabel>Tags</FormLabel>
            {isEditing ? (
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
            ) : (
              <Flex wrap="wrap" gap={2}>
                {video.tags.length > 0 ? (
                  video.tags.map((tag) => (
                    <Badge key={tag} colorScheme="blue" color="white">
                      {tag}
                    </Badge>
                  ))
                ) : (
                  <Text color="gray.500">No tags</Text>
                )}
              </Flex>
            )}
          </FormControl>

          <FormControl>
            <FormLabel>People</FormLabel>
            {isEditing ? (
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
            ) : (
              <Flex wrap="wrap" gap={2}>
                {video.people.length > 0 ? (
                  video.people.map((person) => (
                    <Badge key={person} colorScheme="green" color="white">
                      {person}
                    </Badge>
                  ))
                ) : (
                  <Text color="gray.500">No people</Text>
                )}
              </Flex>
            )}
          </FormControl>
        </VStack>
      </Flex>

      {/* Delete confirmation modal */}
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Delete Video</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            Are you sure you want to delete this video? This action cannot be undone.
          </ModalBody>
          <ModalFooter>
            <Button variant="ghost" mr={3} onClick={onClose}>
              Cancel
            </Button>
            <Button
              colorScheme="red"
              onClick={handleDelete}
              isLoading={deleting}
            >
              Delete
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </Box>
  );
};

export default VideoDetailPage;
