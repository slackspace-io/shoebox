import React from 'react';
import {
  Box,
  Image,
  Text,
  Heading,
  Badge,
  Flex,
  useColorModeValue,
  HStack,
  Icon
} from '@chakra-ui/react';
import { Link as RouterLink } from 'react-router-dom';
import { FaStar, FaRegStar } from 'react-icons/fa';
import { VideoWithMetadata } from '../api/client';

interface VideoCardProps {
  video: VideoWithMetadata;
}

const VideoCard: React.FC<VideoCardProps> = ({ video }) => {
  const cardBg = useColorModeValue('white', 'gray.800');
  const cardBorder = useColorModeValue('gray.200', 'gray.700');

  // Format file size
  const formatFileSize = (bytes?: number): string => {
    if (!bytes) return 'Unknown size';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }

    return `${size.toFixed(1)} ${units[unitIndex]}`;
  };

  // Format date
  const formatDate = (dateString?: string): string => {
    if (!dateString) return 'Unknown date';
    try {
      const date = new Date(dateString);
      if (isNaN(date.getTime())) return 'Unknown date';
      return date.toLocaleDateString();
    } catch (e) {
      return 'Unknown date';
    }
  };

  // Format duration
  const formatDuration = (seconds?: number): string => {
    if (!seconds) return '';

    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const remainingSeconds = seconds % 60;

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`;
    } else {
      return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
    }
  };

  // Render rating stars
  const renderRating = (rating?: number) => {
    if (!rating) return null;

    const stars = [];
    for (let i = 1; i <= 5; i++) {
      stars.push(
        <Icon
          key={i}
          as={i <= rating ? FaStar : FaRegStar}
          color={i <= rating ? 'yellow.400' : 'gray.400'}
          boxSize={3}
        />
      );
    }

    return (
      <HStack spacing={1} mt={1}>
        {stars}
      </HStack>
    );
  };

  return (
    <Box
      as={RouterLink}
      to={`/videos/${video.id}`}
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      bg={cardBg}
      borderColor={cardBorder}
      transition="all 0.2s"
      _hover={{ transform: 'translateY(-4px)', shadow: 'md' }}
    >
      <Image
        src={video.thumbnail_path || '/placeholder-thumbnail.jpg'}
        alt={video.title || video.file_name}
        height="180px"
        width="100%"
        objectFit="cover"
        fallbackSrc="/placeholder-thumbnail.jpg"
      />

      <Box p={4}>
        <Heading size="md" noOfLines={1} mb={2}>
          {video.title || video.file_name}
        </Heading>

        {renderRating(video.rating)}

        <Text fontSize="sm" color="gray.500" mt={2} noOfLines={1}>
          {formatDate(video.created_date)}
        </Text>

        <Flex fontSize="sm" color="gray.500" noOfLines={1} justifyContent="space-between">
          <Text>{formatFileSize(video.file_size)}</Text>
          {video.duration && <Text>{formatDuration(video.duration)}</Text>}
        </Flex>

        {video.tags.length > 0 && (
          <Flex mt={3} flexWrap="wrap" gap={2}>
            {video.tags.slice(0, 3).map((tag) => (
              <Badge key={tag} colorScheme="blue" fontSize="xs" color="white">
                {tag}
              </Badge>
            ))}
            {video.tags.length > 3 && (
              <Badge colorScheme="gray" fontSize="xs" color="white">
                +{video.tags.length - 3} more
              </Badge>
            )}
          </Flex>
        )}

        {video.people.length > 0 && (
          <Flex mt={2} flexWrap="wrap" gap={2}>
            {video.people.slice(0, 2).map((person) => (
              <Badge key={person} colorScheme="green" fontSize="xs" color="white">
                {person}
              </Badge>
            ))}
            {video.people.length > 2 && (
              <Badge colorScheme="gray" fontSize="xs" color="white">
                +{video.people.length - 2} more
              </Badge>
            )}
          </Flex>
        )}
      </Box>
    </Box>
  );
};

export default VideoCard;
