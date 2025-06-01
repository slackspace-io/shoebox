import React, { useState, useEffect } from 'react';
import {
  Box,
  Flex,
  Heading,
  Select as ChakraSelect,
  Button,
  useDisclosure,
  Collapse,
  SimpleGrid,
  useColorModeValue,
  Checkbox,
  Input,
  Text
} from '@chakra-ui/react';
import { FaFilter, FaChevronDown, FaChevronUp } from 'react-icons/fa';
import ReactSelect from 'react-select';
import { tagApi, personApi, locationApi, eventApi, TagUsage, PersonUsage, LocationUsage, EventUsage, VideoSearchParams } from '../api/client';

interface SearchFiltersProps {
  onFilterChange: (filters: Partial<VideoSearchParams>) => void;
  initialFilters?: Partial<VideoSearchParams>;
}

interface SelectOption {
  value: string;
  label: string;
  count?: number;
}

const SearchFilters: React.FC<SearchFiltersProps> = ({ onFilterChange, initialFilters }) => {
  const { isOpen, onToggle } = useDisclosure();
  const [tags, setTags] = useState<SelectOption[]>([]);
  const [people, setPeople] = useState<SelectOption[]>([]);
  const [locations, setLocations] = useState<SelectOption[]>([]);
  const [events, setEvents] = useState<SelectOption[]>([]);
  const [selectedTags, setSelectedTags] = useState<SelectOption[]>([]);
  const [selectedPeople, setSelectedPeople] = useState<SelectOption[]>([]);
  const [selectedLocation, setSelectedLocation] = useState<SelectOption | null>(null);
  const [selectedEvent, setSelectedEvent] = useState<SelectOption | null>(null);
  const [selectedRating, setSelectedRating] = useState<string>('');
  const [isUnreviewed, setIsUnreviewed] = useState<boolean>(false);
  const [sortBy, setSortBy] = useState<string>('created_date');
  const [sortOrder, setSortOrder] = useState<string>('DESC');
  const [startDate, setStartDate] = useState<string>('');
  const [endDate, setEndDate] = useState<string>('');
  const [minDuration, setMinDuration] = useState<string>('');
  const [maxDuration, setMaxDuration] = useState<string>('');
  const [loading, setLoading] = useState(true);

  const bgColor = useColorModeValue('white', 'gray.800');
  const borderColor = useColorModeValue('gray.200', 'gray.700');

  // Load tags, people, locations, and events on component mount
  useEffect(() => {
    const fetchFilters = async () => {
      setLoading(true);
      try {
        // Fetch tags with usage count
        const tagsData = await tagApi.getTagUsage();
        const tagOptions = tagsData.map((tag: TagUsage) => ({
          value: tag.name,
          label: `${tag.name} (${tag.video_count})`,
          count: tag.video_count
        }));
        setTags(tagOptions);

        // Fetch people with usage count
        const peopleData = await personApi.getPersonUsage();
        const peopleOptions = peopleData.map((person: PersonUsage) => ({
          value: person.name,
          label: `${person.name} (${person.video_count})`,
          count: person.video_count
        }));
        setPeople(peopleOptions);

        // Fetch locations with usage count
        const locationsData = await locationApi.getLocationUsage();
        const locationOptions = locationsData.map((location: LocationUsage) => ({
          value: location.name,
          label: `${location.name} (${location.video_count})`,
          count: location.video_count
        }));
        setLocations(locationOptions);

        // Fetch events with usage count
        const eventsData = await eventApi.getEventUsage();
        const eventOptions = eventsData.map((event: EventUsage) => ({
          value: event.name,
          label: `${event.name} (${event.video_count})`,
          count: event.video_count
        }));
        setEvents(eventOptions);
      } catch (error) {
        console.error('Error fetching filters:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchFilters();
  }, []);

  // Initialize filters from props
  useEffect(() => {
    if (initialFilters) {
      // Initialize date filters if provided
      if (initialFilters.start_date) {
        setStartDate(initialFilters.start_date);
      }
      if (initialFilters.end_date) {
        setEndDate(initialFilters.end_date);
      }

      // Initialize location filter if provided
      if (initialFilters.location && locations.length > 0) {
        const locationOption = locations.find(loc => loc.value === initialFilters.location);
        if (locationOption) {
          setSelectedLocation(locationOption);
        }
      }

      // Initialize event filter if provided
      if (initialFilters.event && events.length > 0) {
        const eventOption = events.find(evt => evt.value === initialFilters.event);
        if (eventOption) {
          setSelectedEvent(eventOption);
        }
      }
    }
  }, [initialFilters, locations, events]);

  // Apply filters
  const applyFilters = () => {
    onFilterChange({
      tags: selectedTags.map(tag => tag.value),
      people: selectedPeople.map(person => person.value),
      location: selectedLocation ? selectedLocation.value : undefined,
      event: selectedEvent ? selectedEvent.value : undefined,
      rating: selectedRating ? parseInt(selectedRating, 10) : undefined,
      unreviewed: isUnreviewed || undefined,
      sort_by: sortBy || undefined,
      sort_order: sortOrder || undefined,
      start_date: startDate || undefined,
      end_date: endDate || undefined,
      min_duration: minDuration ? parseInt(minDuration, 10) : undefined,
      max_duration: maxDuration ? parseInt(maxDuration, 10) : undefined
    });
  };

  // Reset filters
  const resetFilters = () => {
    setSelectedTags([]);
    setSelectedPeople([]);
    setSelectedLocation(null);
    setSelectedEvent(null);
    setSelectedRating('');
    setIsUnreviewed(false);
    setSortBy('created_date');
    setSortOrder('DESC');
    setStartDate('');
    setEndDate('');
    setMinDuration('');
    setMaxDuration('');
    onFilterChange({
      tags: undefined,
      people: undefined,
      location: undefined,
      event: undefined,
      rating: undefined,
      unreviewed: undefined,
      sort_by: 'created_date',
      sort_order: 'DESC',
      start_date: undefined,
      end_date: undefined,
      min_duration: undefined,
      max_duration: undefined
    });
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

  return (
    <Box mb={6} borderWidth="1px" borderRadius="lg" p={4} bg={bgColor} borderColor={borderColor}>
      <Flex justify="space-between" align="center" onClick={onToggle} cursor="pointer">
        <Heading size="md" display="flex" alignItems="center">
          <FaFilter style={{ marginRight: '8px' }} />
          Filters
        </Heading>
        <Box>
          {isOpen ? <FaChevronUp /> : <FaChevronDown />}
        </Box>
      </Flex>

      <Collapse in={isOpen} animateOpacity>
        <SimpleGrid columns={{ base: 1, md: 3 }} spacing={4} mt={4}>
          <Box>
            <Heading size="sm" mb={2}>Tags</Heading>
            <ReactSelect
              isMulti
              options={tags}
              value={selectedTags}
              onChange={(selected: any) => setSelectedTags(selected || [])}
              placeholder="Select tags..."
              isLoading={loading}
              styles={selectStyles}
            />
          </Box>

          <Box>
            <Heading size="sm" mb={2}>People</Heading>
            <ReactSelect
              isMulti
              options={people}
              value={selectedPeople}
              onChange={(selected: any) => setSelectedPeople(selected || [])}
              placeholder="Select people..."
              isLoading={loading}
              styles={selectStyles}
            />
          </Box>

          <Box>
            <Heading size="sm" mb={2}>Rating</Heading>
            <ChakraSelect
              value={selectedRating}
              onChange={(e) => setSelectedRating(e.target.value)}
              placeholder="Any rating"
            >
              <option value="1">1 star</option>
              <option value="2">2 stars</option>
              <option value="3">3 stars</option>
              <option value="4">4 stars</option>
              <option value="5">5 stars</option>
            </ChakraSelect>
          </Box>
        </SimpleGrid>

        <SimpleGrid columns={{ base: 1, md: 2 }} spacing={4} mt={4}>
          <Box>
            <Heading size="sm" mb={2}>Location</Heading>
            <ReactSelect
              options={locations}
              value={selectedLocation}
              onChange={(selected: any) => setSelectedLocation(selected)}
              placeholder="Select location..."
              isLoading={loading}
              styles={selectStyles}
              isClearable
            />
          </Box>

          <Box>
            <Heading size="sm" mb={2}>Event</Heading>
            <ReactSelect
              options={events}
              value={selectedEvent}
              onChange={(selected: any) => setSelectedEvent(selected)}
              placeholder="Select event..."
              isLoading={loading}
              styles={selectStyles}
              isClearable
            />
          </Box>
        </SimpleGrid>

        <SimpleGrid columns={{ base: 1, md: 2 }} spacing={4} mt={4}>
          <Box>
            <Heading size="sm" mb={2}>Start Date</Heading>
            <Input
              type="date"
              value={startDate}
              onChange={(e) => setStartDate(e.target.value)}
              placeholder="Start date"
            />
            <Text fontSize="xs" color="gray.500" mt={1}>
              Filter videos created on or after this date
            </Text>
          </Box>

          <Box>
            <Heading size="sm" mb={2}>End Date</Heading>
            <Input
              type="date"
              value={endDate}
              onChange={(e) => setEndDate(e.target.value)}
              placeholder="End date"
            />
            <Text fontSize="xs" color="gray.500" mt={1}>
              Filter videos created on or before this date
            </Text>
          </Box>
        </SimpleGrid>

        <SimpleGrid columns={{ base: 1, md: 2 }} spacing={4} mt={4}>
          <Box>
            <Heading size="sm" mb={2}>Min Duration (seconds)</Heading>
            <Input
              type="number"
              value={minDuration}
              onChange={(e) => setMinDuration(e.target.value)}
              placeholder="Minimum duration"
              min="0"
            />
            <Text fontSize="xs" color="gray.500" mt={1}>
              Filter videos with duration greater than or equal to this value
            </Text>
          </Box>

          <Box>
            <Heading size="sm" mb={2}>Max Duration (seconds)</Heading>
            <Input
              type="number"
              value={maxDuration}
              onChange={(e) => setMaxDuration(e.target.value)}
              placeholder="Maximum duration"
              min="0"
            />
            <Text fontSize="xs" color="gray.500" mt={1}>
              Filter videos with duration less than or equal to this value
            </Text>
          </Box>
        </SimpleGrid>

        <SimpleGrid columns={{ base: 1, md: 2 }} spacing={4} mt={4}>
          <Box>
            <Heading size="sm" mb={2}>Sort By</Heading>
            <ChakraSelect
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value)}
              placeholder="Default (Created Date)"
            >
              <option value="created_date">Created Date</option>
              <option value="duration">Duration</option>
              <option value="title">Title</option>
              <option value="rating">Rating</option>
              <option value="file_size">File Size</option>
            </ChakraSelect>
          </Box>

          <Box>
            <Heading size="sm" mb={2}>Sort Order</Heading>
            <ChakraSelect
              value={sortOrder}
              onChange={(e) => setSortOrder(e.target.value)}
            >
              <option value="ASC">Ascending</option>
              <option value="DESC">Descending</option>
            </ChakraSelect>
          </Box>
        </SimpleGrid>

        <Box mt={4}>
          <Checkbox
            isChecked={isUnreviewed}
            onChange={(e) => setIsUnreviewed(e.target.checked)}
            colorScheme="blue"
          >
            Show only unreviewed videos
          </Checkbox>
        </Box>

        <Flex mt={4} justify="flex-end" gap={2}>
          <Button variant="outline" onClick={resetFilters}>
            Reset
          </Button>
          <Button colorScheme="blue" onClick={applyFilters}>
            Apply Filters
          </Button>
        </Flex>
      </Collapse>
    </Box>
  );
};

export default SearchFilters;
