import React, { useState, useEffect } from 'react';
import {
  Box,
  Heading,
  Button,
  Flex,
  Input,
  InputGroup,
  InputLeftElement,
  useToast,
  Text,
  Spinner,
  SimpleGrid
} from '@chakra-ui/react';
import { FaSearch, FaSync } from 'react-icons/fa';
import { videoApi, scanApi, VideoWithMetadata, VideoSearchParams } from '../api/client';
import VideoCard from '../components/VideoCard';
import SearchFilters from '../components/SearchFilters';
import { useScanContext } from '../contexts/ScanContext';

const HomePage: React.FC = () => {
  const [videos, setVideos] = useState<VideoWithMetadata[]>([]);
  const [loading, setLoading] = useState(true);
  const [scanning, setScanning] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [searchParams, setSearchParams] = useState<VideoSearchParams>({
    limit: 100,
    offset: 0
  });
  const toast = useToast();
  const { scanStatus, checkScanStatus } = useScanContext();

  // Function to fetch videos
  const fetchVideos = async () => {
    setLoading(true);
    try {
      const results = await videoApi.searchVideos(searchParams);
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

  // Load videos on component mount and when search params change
  useEffect(() => {
    fetchVideos();
  }, [searchParams, toast]);

  // Refresh videos when scan completes
  useEffect(() => {
    // If scan was in progress but is now complete, refresh the videos
    if (!scanStatus.inProgress && scanStatus.newVideosCount > 0) {
      fetchVideos();

      // Show a toast notification about the completed scan
      toast({
        title: 'Scan complete',
        description: `Found ${scanStatus.newVideosCount} new videos and updated ${scanStatus.updatedVideosCount} videos.`,
        status: 'success',
        duration: 5000,
        isClosable: true,
      });
    }
  }, [scanStatus.inProgress]);

  // Handle search input change
  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value);
  };

  // Handle search submit
  const handleSearch = () => {
    setSearchParams({
      ...searchParams,
      query: searchQuery.trim() || undefined,
      offset: 0, // Reset pagination when searching
    });
  };

  // Handle search on Enter key
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  // Handle filter changes
  const handleFilterChange = (newFilters: Partial<VideoSearchParams>) => {
    setSearchParams({
      ...searchParams,
      ...newFilters,
      offset: 0, // Reset pagination when filters change
    });
  };

  // Handle scan directories
  const handleScan = async () => {
    setScanning(true);
    try {
      await scanApi.scanDirectories();

      // Check scan status immediately after starting the scan
      await checkScanStatus();

      toast({
        title: 'Scan started',
        description: 'The scan has been started in the background. You can continue using the application.',
        status: 'info',
        duration: 5000,
        isClosable: true,
      });

      // We'll refresh the videos list automatically when the scan completes
      // via the scan status polling mechanism
    } catch (error) {
      console.error('Error starting scan:', error);
      toast({
        title: 'Error starting scan',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    } finally {
      setScanning(false);
    }
  };

  return (
    <Box>
      <Flex justify="space-between" align="center" mb={6}>
        <Heading size="xl">Videos</Heading>
        <Button
          leftIcon={<FaSync />}
          colorScheme="blue"
          onClick={handleScan}
          isLoading={scanning}
          loadingText="Scanning"
        >
          Scan Directories
        </Button>
      </Flex>

      <Flex mb={6} direction={{ base: 'column', md: 'row' }} gap={4}>
        <InputGroup size="md" flex="1">
          <InputLeftElement pointerEvents="none">
            <FaSearch color="gray.300" />
          </InputLeftElement>
          <Input
            placeholder="Search videos..."
            value={searchQuery}
            onChange={handleSearchChange}
            onKeyDown={handleKeyDown}
          />
        </InputGroup>
        <Button colorScheme="blue" onClick={handleSearch} minW="100px">
          Search
        </Button>
      </Flex>

      <SearchFilters onFilterChange={handleFilterChange} />

      {loading ? (
        <Flex justify="center" align="center" h="200px">
          <Spinner size="xl" />
        </Flex>
      ) : videos.length === 0 ? (
        <Box textAlign="center" py={10}>
          <Text fontSize="xl">No videos found</Text>
          <Text mt={2} color="gray.500">
            Try adjusting your search or scan for new videos
          </Text>
        </Box>
      ) : (
        <SimpleGrid columns={{ base: 1, sm: 2, md: 3, lg: 4 }} spacing={6} mt={6}>
          {videos.map((video) => (
            <VideoCard key={video.id} video={video} />
          ))}
        </SimpleGrid>
      )}
    </Box>
  );
};

export default HomePage;
