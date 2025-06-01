import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Box,
  Flex,
  Heading,
  Text,
  Button,
  VStack,
  HStack,
  FormControl,
  FormLabel,
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
  Code,
  useColorModeValue
} from '@chakra-ui/react';
import { FaEdit, FaSave, FaTrash, FaArrowLeft, FaBug } from 'react-icons/fa';
import ReactPlayer from 'react-player';
import { videoApi, VideoWithMetadata, UpdateVideoDto } from '../api/client';
import VideoForm from '../components/VideoForm';

interface SelectOption {
  value: string;
  label: string;
}

const VideoDetailPage: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { isOpen: isDebugOpen, onOpen: onDebugOpen, onClose: onDebugClose } = useDisclosure();

  const [video, setVideo] = useState<VideoWithMetadata | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [rawDatabaseValues, setRawDatabaseValues] = useState<string>('');

  // Form state
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [rating, setRating] = useState<number | undefined>(undefined);
  const [location, setLocation] = useState('');
  const [event, setEvent] = useState('');
  const [selectedTags, setSelectedTags] = useState<SelectOption[]>([]);
  const [selectedPeople, setSelectedPeople] = useState<SelectOption[]>([]);


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
        setLocation(videoData.location || '');
        setEvent(videoData.event || '');
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


  // Handle save
  const handleSave = async () => {
    if (!id || !video) return;

    setSaving(true);
    try {
      const updateData: UpdateVideoDto = {
        title: title || undefined,
        description: description || undefined,
        rating,
        location: location || undefined,
        event: event || undefined,
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
        setLocation(video.location || '');
        setEvent(video.event || '');
        setSelectedTags(video.tags.map(tag => ({ value: tag, label: tag })));
        setSelectedPeople(video.people.map(person => ({ value: person, label: person })));
      }
    }
    setIsEditing(!isEditing);
  };

  // Handle showing debug information
  const handleShowDebug = async () => {
    if (!id) return;

    try {
      // Fetch the latest data from the server
      const videoData = await videoApi.getVideo(id);
      setRawDatabaseValues(JSON.stringify(videoData, null, 2));
      onDebugOpen();
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
              url={`/api/videos/${video.id}/stream`}
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
          {isEditing ? (
            <VideoForm
              video={video}
              formData={{
                title,
                description,
                rating,
                location: location,
                event,
                selectedTags,
                selectedPeople
              }}
              onChange={(formData) => {
                if (formData.title !== undefined) setTitle(formData.title);
                if (formData.description !== undefined) setDescription(formData.description);
                if (formData.rating !== undefined) setRating(formData.rating);
                if (formData.location !== undefined) setLocation(formData.location);
                if (formData.event !== undefined) setEvent(formData.event);
                if (formData.tags !== undefined) {
                  setSelectedTags(formData.tags.map(tag => ({ value: tag, label: tag })));
                }
                if (formData.people !== undefined) {
                  setSelectedPeople(formData.people.map(person => ({ value: person, label: person })));
                }
              }}
            />
          ) : (
            <>
              <Heading size="xl">{video.title || video.file_name}</Heading>

              <FormControl>
                <FormLabel>Rating</FormLabel>
                <Flex>
                  {Array.from({ length: 5 }).map((_, i) => (
                    <Text key={i} color={i < (rating || 0) ? "yellow.400" : "gray.400"} fontSize="xl">
                      ★
                    </Text>
                  ))}
                </Flex>
              </FormControl>

              <FormControl>
                <FormLabel>Description</FormLabel>
                <Text>{video.description || 'No description'}</Text>
              </FormControl>

              <FormControl>
                <FormLabel>Location</FormLabel>
                <Text>{video.location || 'No location'}</Text>
              </FormControl>

              <FormControl>
                <FormLabel>Event</FormLabel>
                <Text>{video.event || 'No event'}</Text>
              </FormControl>

              <FormControl>
                <FormLabel>Tags</FormLabel>
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
              </FormControl>

              <FormControl>
                <FormLabel>People</FormLabel>
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
              </FormControl>
            </>
          )}
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

      {/* Debug Modal */}
      <Modal isOpen={isDebugOpen} onClose={() => {
        setRawDatabaseValues('');
        onDebugClose();
      }} size="xl">
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
            <Button colorScheme="blue" mr={3} onClick={() => {
              setRawDatabaseValues('');
              onDebugClose();
            }}>
              Close
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </Box>
  );
};

export default VideoDetailPage;
