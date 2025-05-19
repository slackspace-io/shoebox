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
  useColorModeValue
} from '@chakra-ui/react';
import { FaFilter, FaChevronDown, FaChevronUp } from 'react-icons/fa';
import ReactSelect from 'react-select';
import { tagApi, personApi, TagUsage, PersonUsage, VideoSearchParams } from '../api/client';

interface SearchFiltersProps {
  onFilterChange: (filters: Partial<VideoSearchParams>) => void;
}

interface SelectOption {
  value: string;
  label: string;
  count?: number;
}

const SearchFilters: React.FC<SearchFiltersProps> = ({ onFilterChange }) => {
  const { isOpen, onToggle } = useDisclosure();
  const [tags, setTags] = useState<SelectOption[]>([]);
  const [people, setPeople] = useState<SelectOption[]>([]);
  const [selectedTags, setSelectedTags] = useState<SelectOption[]>([]);
  const [selectedPeople, setSelectedPeople] = useState<SelectOption[]>([]);
  const [selectedRating, setSelectedRating] = useState<string>('');
  const [loading, setLoading] = useState(true);

  const bgColor = useColorModeValue('white', 'gray.800');
  const borderColor = useColorModeValue('gray.200', 'gray.700');

  // Load tags and people on component mount
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
      } catch (error) {
        console.error('Error fetching filters:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchFilters();
  }, []);

  // Apply filters
  const applyFilters = () => {
    onFilterChange({
      tags: selectedTags.map(tag => tag.value),
      people: selectedPeople.map(person => person.value),
      rating: selectedRating ? parseInt(selectedRating, 10) : undefined
    });
  };

  // Reset filters
  const resetFilters = () => {
    setSelectedTags([]);
    setSelectedPeople([]);
    setSelectedRating('');
    onFilterChange({
      tags: undefined,
      people: undefined,
      rating: undefined
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
        : base.backgroundColor,
      color: useColorModeValue('black', 'white')
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
              <option value="">Any rating</option>
              <option value="1">1 star</option>
              <option value="2">2 stars</option>
              <option value="3">3 stars</option>
              <option value="4">4 stars</option>
              <option value="5">5 stars</option>
            </ChakraSelect>
          </Box>
        </SimpleGrid>

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
