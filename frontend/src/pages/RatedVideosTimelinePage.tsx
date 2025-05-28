import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Box,
  Heading,
  Text,
  Flex,
  Select,
  Checkbox,
  Spinner,
  useToast,
  Alert,
  AlertIcon,
  AlertTitle,
  AlertDescription,
  useColorModeValue,
  Tooltip,
} from '@chakra-ui/react';
import { videoApi, VideoWithMetadata, VideoSearchParams } from '../api/client';

// Define time period options
const TIME_PERIODS = [
  { value: '3', label: 'Last 3 Months' },
  { value: '6', label: 'Last 6 Months' },
  { value: '12', label: 'Last 12 Months' },
  { value: '24', label: 'Last 2 Years' },
  { value: '60', label: 'Last 5 Years' },
  { value: 'all', label: 'All Time' },
];

// Define grouping options
const GROUPING_OPTIONS = [
  { value: 'auto', label: 'Auto' },
  { value: 'day', label: 'Day' },
  { value: 'month', label: 'Month' },
  { value: 'quarter', label: 'Quarter' },
  { value: 'year', label: 'Year' },
];

// Interface for grouped video data
interface GroupedVideoData {
  period: string;
  videos: VideoWithMetadata[];
  maxRating: number;
  avgRating: number;
  count: number;
}

const RatedVideosTimelinePage: React.FC = () => {
  const navigate = useNavigate();
  const [videos, setVideos] = useState<VideoWithMetadata[]>([]);
  const [groupedData, setGroupedData] = useState<GroupedVideoData[]>([]);
  const [timePeriod, setTimePeriod] = useState<string>('12'); // Default to 12 months
  const [grouping, setGrouping] = useState<string>('auto'); // Default to auto grouping
  const [showUnreviewed, setShowUnreviewed] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(true);
  const toast = useToast();

  const bgColor = useColorModeValue('white', 'gray.800');
  const borderColor = useColorModeValue('gray.200', 'gray.700');
  const barEmptyColor = useColorModeValue('gray.100', 'gray.700');

  // Fetch videos when component mounts or filters change
  useEffect(() => {
    fetchVideos();
  }, [timePeriod, showUnreviewed]);

  // Fetch videos from API
  const fetchVideos = async () => {
    setLoading(true);
    try {
      // Calculate date range based on selected time period
      const endDate = new Date();
      let startDate: Date | null = null;

      if (timePeriod !== 'all') {
        startDate = new Date();
        startDate.setMonth(startDate.getMonth() - parseInt(timePeriod));
      }

      // Prepare search params
      const params: VideoSearchParams = {
        limit: 1000, // Get a large number of videos
        unreviewed: showUnreviewed ? undefined : false, // If showUnreviewed is false, exclude unreviewed videos
      };

      // Add date range if not "all time"
      if (startDate) {
        (params as any).start_date = startDate.toISOString().split('T')[0];
        (params as any).end_date = endDate.toISOString().split('T')[0];
      }

      const results = await videoApi.searchVideos(params);

      // Filter out videos without ratings if not showing unreviewed
      const filteredVideos = showUnreviewed
        ? results
        : results.filter(video => video.rating !== undefined && video.rating > 0);

      setVideos(filteredVideos);

      // Group the videos based on the selected grouping
      groupVideos(filteredVideos);
    } catch (error) {
      console.error('Error fetching videos:', error);
      toast({
        title: 'Error fetching videos',
        description: 'There was an error fetching the videos. Please try again.',
        status: 'error',
        duration: 5000,
        isClosable: true,
      });
    } finally {
      setLoading(false);
    }
  };

  // Group videos based on selected grouping
  const groupVideos = (videos: VideoWithMetadata[]) => {
    if (videos.length === 0) {
      setGroupedData([]);
      return;
    }

    // Determine appropriate grouping if auto is selected
    let effectiveGrouping = grouping;
    if (grouping === 'auto') {
      // Logic to determine the best grouping based on date range
      const dateRange = parseInt(timePeriod);
      if (timePeriod === 'all' || dateRange > 24) {
        effectiveGrouping = 'year';
      } else if (dateRange > 6) {
        effectiveGrouping = 'quarter';
      } else if (dateRange > 1) {
        effectiveGrouping = 'month';
      } else {
        effectiveGrouping = 'day';
      }
    }

    // Group videos by the determined period
    const grouped: Record<string, VideoWithMetadata[]> = {};

    videos.forEach(video => {
      if (!video.created_date) return;

      const date = new Date(video.created_date);
      let periodKey: string;

      switch (effectiveGrouping) {
        case 'day':
          periodKey = date.toISOString().split('T')[0]; // YYYY-MM-DD
          break;
        case 'month':
          periodKey = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}`; // YYYY-MM
          break;
        case 'quarter':
          const quarter = Math.floor(date.getMonth() / 3) + 1;
          periodKey = `${date.getFullYear()}-Q${quarter}`; // YYYY-Q#
          break;
        case 'year':
        default:
          periodKey = `${date.getFullYear()}`; // YYYY
          break;
      }

      if (!grouped[periodKey]) {
        grouped[periodKey] = [];
      }

      grouped[periodKey].push(video);
    });

    // Convert grouped object to array and calculate stats
    const groupedArray: GroupedVideoData[] = Object.keys(grouped)
      .sort() // Sort periods chronologically
      .map(period => {
        const periodVideos = grouped[period];
        const ratings = periodVideos
          .map(v => v.rating || 0)
          .filter(r => r > 0); // Filter out unrated videos for average calculation

        const maxRating = Math.max(...periodVideos.map(v => v.rating || 0));
        const avgRating = ratings.length > 0
          ? ratings.reduce((sum, rating) => sum + rating, 0) / ratings.length
          : 0;

        return {
          period,
          videos: periodVideos,
          maxRating,
          avgRating,
          count: periodVideos.length,
        };
      });

    setGroupedData(groupedArray);
  };

  // Update grouping when it changes
  useEffect(() => {
    if (videos.length > 0) {
      groupVideos(videos);
    }
  }, [grouping]);

  // Format period label based on grouping
  const formatPeriodLabel = (period: string, effectiveGrouping: string): string => {
    switch (effectiveGrouping) {
      case 'day':
        return new Date(period).toLocaleDateString();
      case 'month': {
        const [year, month] = period.split('-');
        return `${new Date(parseInt(year), parseInt(month) - 1).toLocaleString('default', { month: 'short' })} ${year}`;
      }
      case 'quarter': {
        const [year, quarter] = period.split('-Q');
        return `${quarter}Q ${year}`;
      }
      case 'year':
        return period;
      default:
        return period;
    }
  };

  // Determine effective grouping for display
  const getEffectiveGrouping = (): string => {
    if (grouping !== 'auto') return grouping;

    // Logic to determine the best grouping based on date range
    const dateRange = parseInt(timePeriod);
    if (timePeriod === 'all' || dateRange > 24) {
      return 'year';
    } else if (dateRange > 6) {
      return 'quarter';
    } else if (dateRange > 1) {
      return 'month';
    } else {
      return 'day';
    }
  };

  // Get color for rating bar
  const getRatingColor = (rating: number): string => {
    if (rating >= 4.5) return 'green.500';
    if (rating >= 3.5) return 'teal.500';
    if (rating >= 2.5) return 'blue.500';
    if (rating >= 1.5) return 'yellow.500';
    return 'red.500';
  };

  // Handle click on a timeline bar
  const handleBarClick = (group: GroupedVideoData) => {
    // Determine date range based on the grouping
    let startDate: string;
    let endDate: string;

    const effectiveGrouping = getEffectiveGrouping();

    switch (effectiveGrouping) {
      case 'day':
        // For day grouping, use the same day
        startDate = group.period;
        endDate = group.period;
        break;
      case 'month': {
        // For month grouping, use the first and last day of the month
        const [year, month] = group.period.split('-');
        const firstDay = new Date(parseInt(year), parseInt(month) - 1, 1);
        const lastDay = new Date(parseInt(year), parseInt(month), 0); // Last day of the month
        startDate = firstDay.toISOString().split('T')[0];
        endDate = lastDay.toISOString().split('T')[0];
        break;
      }
      case 'quarter': {
        // For quarter grouping, use the first and last day of the quarter
        const [year, quarter] = group.period.split('-Q');
        const quarterNum = parseInt(quarter);
        const firstMonth = (quarterNum - 1) * 3;
        const firstDay = new Date(parseInt(year), firstMonth, 1);
        const lastDay = new Date(parseInt(year), firstMonth + 3, 0); // Last day of the last month in the quarter
        startDate = firstDay.toISOString().split('T')[0];
        endDate = lastDay.toISOString().split('T')[0];
        break;
      }
      case 'year': {
        // For year grouping, use the first and last day of the year
        const year = parseInt(group.period);
        const firstDay = new Date(year, 0, 1);
        const lastDay = new Date(year, 11, 31);
        startDate = firstDay.toISOString().split('T')[0];
        endDate = lastDay.toISOString().split('T')[0];
        break;
      }
      default:
        // Default case, shouldn't happen
        startDate = group.period;
        endDate = group.period;
    }

    // Navigate to the videos page with date range filter
    navigate(`/?start_date=${startDate}&end_date=${endDate}`);
  };

  return (
    <Box>
      <Heading size="xl" mb={6}>Rated Videos Timeline</Heading>

      <Box p={4} borderWidth="1px" borderRadius="lg" bg={bgColor} borderColor={borderColor} mb={6}>
        <Heading size="md" mb={4}>Timeline Settings</Heading>

        <Flex direction={{ base: 'column', md: 'row' }} gap={4} mb={4}>
          <Box flex="1">
            <Text mb={2}>Time Period</Text>
            <Select
              value={timePeriod}
              onChange={(e) => setTimePeriod(e.target.value)}
            >
              {TIME_PERIODS.map(option => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </Select>
          </Box>

          <Box flex="1">
            <Text mb={2}>Group By</Text>
            <Select
              value={grouping}
              onChange={(e) => setGrouping(e.target.value)}
            >
              {GROUPING_OPTIONS.map(option => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </Select>
          </Box>
        </Flex>

        <Checkbox
          isChecked={showUnreviewed}
          onChange={(e) => setShowUnreviewed(e.target.checked)}
        >
          Show unreviewed videos
        </Checkbox>
      </Box>

      {loading ? (
        <Flex justify="center" align="center" h="200px">
          <Spinner size="xl" />
        </Flex>
      ) : groupedData.length === 0 ? (
        <Alert status="info">
          <AlertIcon />
          <AlertTitle>No videos found</AlertTitle>
          <AlertDescription>
            No rated videos were found for the selected time period.
            {!showUnreviewed && " Try enabling 'Show unreviewed videos' to see all videos."}
          </AlertDescription>
        </Alert>
      ) : (
        <Box p={4} borderWidth="1px" borderRadius="lg" bg={bgColor} borderColor={borderColor}>
          <Heading size="md" mb={4}>
            Video Ratings Over Time
            <Text as="span" fontWeight="normal" fontSize="md" ml={2}>
              (Grouped by {getEffectiveGrouping()})
            </Text>
          </Heading>

          {/* Timeline start/end dates */}
          <Flex justify="space-between" mb={2}>
            <Text fontSize="sm" fontWeight="bold">
              {groupedData.length > 0 ? formatPeriodLabel(groupedData[0].period, getEffectiveGrouping()) : 'Start'}
            </Text>
            <Text fontSize="sm" fontWeight="bold">
              {groupedData.length > 0 ? formatPeriodLabel(groupedData[groupedData.length - 1].period, getEffectiveGrouping()) : 'End'}
            </Text>
          </Flex>

          {/* Horizontal timeline */}
          <Box position="relative" mb={10}>
            <Flex
              direction="row"
              w="100%"
              h="100px"
              bg={barEmptyColor}
              borderRadius="md"
              overflow="hidden"
              position="relative"
              justify="space-between"
              align="flex-end"
              px={1}
            >
              {groupedData.map((group) => {
                // Calculate size factor based on max rating and video count
                const sizeFactor = Math.max(
                  (group.maxRating / 5) * 0.7 + (Math.min(group.count, 20) / 20) * 0.3,
                  0.1 // Minimum size factor
                );

                // Calculate width - fixed width with slight variation based on rating/count
                const baseWidth = 100 / Math.max(groupedData.length, 1);
                const width = Math.max(baseWidth * 0.8, 0.5); // Ensure minimum width

                return (
                  <Tooltip
                    key={group.period}
                    label={
                      <Box p={1}>
                        <Text fontWeight="bold">{formatPeriodLabel(group.period, getEffectiveGrouping())}</Text>
                        <Text>Max Rating: {group.maxRating.toFixed(1)}</Text>
                        <Text>Avg Rating: {group.avgRating.toFixed(1)}</Text>
                        <Text>Videos: {group.count}</Text>
                        {group.videos
                          .sort((a, b) => (b.rating || 0) - (a.rating || 0))
                          .slice(0, 3)
                          .map(video => (
                            <Text key={video.id} fontSize="xs" mt={1}>
                              {video.rating ? `${video.rating.toFixed(1)}★` : 'Unrated'}: {(video.title || video.file_name).substring(0, 20)}
                              {(video.title || video.file_name).length > 20 ? '...' : ''}
                            </Text>
                          ))}
                      </Box>
                    }
                    hasArrow
                    placement="top"
                  >
                    <Box
                      h={`${Math.max(sizeFactor * 100, 10)}%`}
                      w={`${width}%`}
                      minW="4px"
                      maxW="30px"
                      bg={getRatingColor(group.maxRating)}
                      opacity={0.8}
                      borderRadius="md"
                      mx="1px"
                      alignSelf="flex-end"
                      _hover={{ opacity: 1, transform: 'translateY(-2px)' }}
                      transition="all 0.2s ease-in-out"
                      onClick={(e) => {
                        e.stopPropagation(); // Prevent tooltip from interfering
                        handleBarClick(group);
                      }}
                      cursor="pointer" // Add pointer cursor to indicate clickability
                    />
                  </Tooltip>
                );
              })}
            </Flex>

            {/* Period labels below the timeline */}
            <Flex
              direction="row"
              w="100%"
              justify="space-between"
              mt={2}
              px={1}
            >
              {groupedData.length > 0 && (
                <>
                  <Text fontSize="xs">
                    {formatPeriodLabel(groupedData[0].period, getEffectiveGrouping())}
                  </Text>

                  {groupedData.length > 2 && (
                    <Text fontSize="xs">
                      {formatPeriodLabel(groupedData[Math.floor(groupedData.length / 2)].period, getEffectiveGrouping())}
                    </Text>
                  )}

                  <Text fontSize="xs">
                    {formatPeriodLabel(groupedData[groupedData.length - 1].period, getEffectiveGrouping())}
                  </Text>
                </>
              )}
            </Flex>
          </Box>

          {/* Legend */}
          <Flex mt={10} justify="space-between" wrap="wrap">
            <Box>
              <Text fontSize="sm" fontWeight="bold">Color Legend:</Text>
              <Flex mt={1} align="center">
                {[1, 2, 3, 4, 5].map(rating => (
                  <Flex key={rating} align="center" mr={3}>
                    <Box w="12px" h="12px" bg={getRatingColor(rating)} mr={1} borderRadius="sm" />
                    <Text fontSize="xs">{rating}★</Text>
                  </Flex>
                ))}
              </Flex>
            </Box>
            <Box>
              <Text fontSize="sm" fontWeight="bold">Size represents:</Text>
              <Text fontSize="xs">Higher ratings and more videos = larger blocks</Text>
            </Box>
          </Flex>
        </Box>
      )}
    </Box>
  );
};

export default RatedVideosTimelinePage;
