import React, { useState, useEffect } from 'react';
import {
  Box,
  Heading,
  Text,
  Button,
  Input,
  FormControl,
  FormLabel,
  FormHelperText,
  Flex,
  VStack,
  HStack,
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
  useDisclosure,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
} from '@chakra-ui/react';
import { FaSearch, FaFileExport, FaCheck, FaTimes } from 'react-icons/fa';
import { videoApi, exportApi, VideoWithMetadata } from '../api/client';
import SearchFilters from '../components/SearchFilters';

const ExportPage: React.FC = () => {
  const [videos, setVideos] = useState<VideoWithMetadata[]>([]);
  const [selectedVideos, setSelectedVideos] = useState<string[]>([]);
  const [projectName, setProjectName] = useState('');
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(true);
  const [exporting, setExporting] = useState(false);
  const [exportPath, setExportPath] = useState<string | null>(null);
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const bgColor = useColorModeValue('white', 'gray.800');
  const borderColor = useColorModeValue('gray.200', 'gray.700');

  // Load videos on component mount
  useEffect(() => {
    fetchVideos();
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

  // Handle export
  const handleExport = async () => {
    if (selectedVideos.length === 0) {
      toast({
        title: 'No videos selected',
        description: 'Please select at least one video to export',
        status: 'warning',
        duration: 3000,
        isClosable: true,
      });
      return;
    }

    if (!projectName.trim()) {
      toast({
        title: 'Project name required',
        description: 'Please enter a project name',
        status: 'warning',
        duration: 3000,
        isClosable: true,
      });
      return;
    }

    setExporting(true);
    try {
      const result = await exportApi.exportVideos({
        video_ids: selectedVideos,
        project_name: projectName.trim(),
      });

      setExportPath(result.export_path);
      onOpen(); // Open success modal

      // Reset selection after successful export
      setSelectedVideos([]);
      setProjectName('');
    } catch (error) {
      console.error('Error exporting videos:', error);
      toast({
        title: 'Error exporting videos',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    } finally {
      setExporting(false);
    }
  };

  // Format file size
  const formatFileSize = (bytes?: number): string => {
    if (!bytes) return 'Unknown';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }

    return `${size.toFixed(1)} ${units[unitIndex]}`;
  };

  return (
    <Box>
      <Heading size="xl" mb={6}>Export Videos</Heading>

      <VStack spacing={6} align="stretch">
        <Box p={4} borderWidth="1px" borderRadius="lg" bg={bgColor} borderColor={borderColor}>
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
                    <Th>Size</Th>
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
                          {video.file_path}
                        </Text>
                      </Td>
                      <Td>
                        <Flex wrap="wrap" gap={1}>
                          {video.tags.slice(0, 3).map(tag => (
                            <Badge key={tag} colorScheme="blue" fontSize="xs">
                              {tag}
                            </Badge>
                          ))}
                          {video.tags.length > 3 && (
                            <Badge colorScheme="gray" fontSize="xs">
                              +{video.tags.length - 3}
                            </Badge>
                          )}
                        </Flex>
                      </Td>
                      <Td>{formatFileSize(video.file_size)}</Td>
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

        <Box p={4} borderWidth="1px" borderRadius="lg" bg={bgColor} borderColor={borderColor}>
          <Heading size="md" mb={4}>2. Configure Export</Heading>

          <FormControl isRequired>
            <FormLabel>Project Name</FormLabel>
            <Input
              placeholder="Enter project name"
              value={projectName}
              onChange={(e) => setProjectName(e.target.value)}
            />
            <FormHelperText>
              This will be used to create the export folder
            </FormHelperText>
          </FormControl>

          <Button
            mt={6}
            colorScheme="blue"
            leftIcon={<FaFileExport />}
            isLoading={exporting}
            loadingText="Exporting"
            onClick={handleExport}
            isDisabled={selectedVideos.length === 0 || !projectName.trim()}
          >
            Export Selected Videos
          </Button>
        </Box>
      </VStack>

      {/* Success Modal */}
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Export Successful</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <Alert status="success" mb={4}>
              <AlertIcon />
              <Box>
                <AlertTitle>Videos exported successfully!</AlertTitle>
                <AlertDescription>
                  {selectedVideos.length} videos were exported to the project folder.
                </AlertDescription>
              </Box>
            </Alert>

            {exportPath && (
              <Text mt={2}>
                <strong>Export path:</strong> {exportPath}
              </Text>
            )}
          </ModalBody>
          <ModalFooter>
            <Button colorScheme="blue" onClick={onClose}>
              Close
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </Box>
  );
};

export default ExportPage;
