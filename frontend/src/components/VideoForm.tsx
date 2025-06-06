import React, { useState, useEffect } from 'react';
import {
  FormControl,
  FormLabel,
  Input,
  Textarea,
  HStack,
  IconButton,
  useToast,
  useColorModeValue
} from '@chakra-ui/react';
import { FaStar, FaRegStar } from 'react-icons/fa';
import CreatableSelect from 'react-select/creatable';
import { tagApi, personApi, locationApi, eventApi, shoeboxApi, VideoWithMetadata, UpdateVideoDto } from '../api/client';

interface SelectOption {
  value: string;
  label: string;
}

interface VideoFormProps {
  video: VideoWithMetadata;
  onChange: (formData: Partial<UpdateVideoDto>) => void;
  formData: {
    title: string;
    description: string;
    rating?: number;
    location: string;
    event: string;
    selectedTags: SelectOption[];
    selectedPeople: SelectOption[];
    selectedShoeboxes: SelectOption[];
  };
  readOnly?: boolean;
}

const VideoForm: React.FC<VideoFormProps> = ({
  video: _video, // Renamed to _video to indicate it's intentionally unused
  onChange,
  formData,
  readOnly = false
}) => {
  const toast = useToast();

  // Options for select inputs
  const [tagOptions, setTagOptions] = useState<SelectOption[]>([]);
  const [peopleOptions, setPeopleOptions] = useState<SelectOption[]>([]);
  const [locationOptions, setLocationOptions] = useState<SelectOption[]>([]);
  const [eventOptions, setEventOptions] = useState<SelectOption[]>([]);
  const [shoeboxOptions, setShoeboxOptions] = useState<SelectOption[]>([]);

  const bgColor = useColorModeValue('white', 'gray.800');
  const borderColor = useColorModeValue('gray.200', 'gray.700');

  // Load tags, people, locations, events, and shoeboxes options
  useEffect(() => {
    const fetchOptions = async () => {
      try {
        // Fetch tags
        const tags = await tagApi.getTags();
        setTagOptions(tags.map(tag => ({ value: tag.name, label: tag.name })));

        // Fetch people
        const people = await personApi.getPeople();
        setPeopleOptions(people.map(person => ({ value: person.name, label: person.name })));

        // Fetch locations
        const locations = await locationApi.getLocations();
        setLocationOptions(locations.map(location => ({ value: location, label: location })));

        // Fetch events
        const events = await eventApi.getEvents();
        setEventOptions(events.map(event => ({ value: event, label: event })));

        // Fetch shoeboxes
        const shoeboxes = await shoeboxApi.getShoeboxes();
        setShoeboxOptions(shoeboxes.map(shoebox => ({ value: shoebox.name, label: shoebox.name })));
      } catch (error) {
        console.error('Error fetching options:', error);
      }
    };

    fetchOptions();
  }, []);

  // Handle rating change
  const handleRatingChange = (newRating: number) => {
    const updatedRating = newRating === formData.rating ? undefined : newRating;
    onChange({ rating: updatedRating });
  };

  // Render rating stars
  const renderRatingStars = () => {
    const stars = [];
    for (let i = 1; i <= 5; i++) {
      stars.push(
        <IconButton
          key={i}
          icon={i <= (formData.rating || 0) ? <FaStar /> : <FaRegStar />}
          aria-label={`${i} star`}
          variant="ghost"
          color={i <= (formData.rating || 0) ? 'yellow.400' : 'gray.400'}
          isDisabled={readOnly}
          onClick={() => !readOnly && handleRatingChange(i)}
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

  return (
    <>
      <FormControl>
        <FormLabel>Title</FormLabel>
        {readOnly ? (
          <Input
            value={formData.title}
            isReadOnly={true}
          />
        ) : (
          <Input
            value={formData.title}
            onChange={(e) => onChange({ title: e.target.value })}
            placeholder="Enter title"
          />
        )}
      </FormControl>

      <FormControl>
        <FormLabel>Rating</FormLabel>
        {renderRatingStars()}
      </FormControl>

      <FormControl>
        <FormLabel>Description</FormLabel>
        {readOnly ? (
          <Textarea
            value={formData.description}
            isReadOnly={true}
            rows={4}
          />
        ) : (
          <Textarea
            value={formData.description}
            onChange={(e) => onChange({ description: e.target.value })}
            placeholder="Enter description"
            rows={4}
          />
        )}
      </FormControl>

      <FormControl>
        <FormLabel>Location</FormLabel>
        {readOnly ? (
          <Input
            value={formData.location}
            isReadOnly={true}
          />
        ) : (
          <CreatableSelect
            options={locationOptions}
            value={formData.location ? { value: formData.location, label: formData.location } : null}
            onChange={(selected: any) => onChange({
              location: selected ? selected.value : undefined
            })}
            placeholder="Select or create location..."
            styles={selectStyles}
            isClearable
            formatCreateLabel={(inputValue) => `Create location "${inputValue}"`}
            onCreateOption={async (inputValue) => {
              if (readOnly) return;

              // For locations, we don't need to create anything in the backend
              // Just update the form data and add to options
              onChange({ location: inputValue });

              // Add to options if not already present
              if (!locationOptions.some(option => option.value === inputValue)) {
                setLocationOptions([...locationOptions, { value: inputValue, label: inputValue }]);
              }

              toast({
                title: 'Location added',
                status: 'success',
                duration: 2000,
                isClosable: true,
              });
            }}
          />
        )}
      </FormControl>

      <FormControl>
        <FormLabel>Event</FormLabel>
        {readOnly ? (
          <Input
            value={formData.event}
            isReadOnly={true}
          />
        ) : (
          <CreatableSelect
            options={eventOptions}
            value={formData.event ? { value: formData.event, label: formData.event } : null}
            onChange={(selected: any) => onChange({
              event: selected ? selected.value : undefined
            })}
            placeholder="Select or create event..."
            styles={selectStyles}
            isClearable
            formatCreateLabel={(inputValue) => `Create event "${inputValue}"`}
            onCreateOption={async (inputValue) => {
              if (readOnly) return;

              // For events, we don't need to create anything in the backend
              // Just update the form data and add to options
              onChange({ event: inputValue });

              // Add to options if not already present
              if (!eventOptions.some(option => option.value === inputValue)) {
                setEventOptions([...eventOptions, { value: inputValue, label: inputValue }]);
              }

              toast({
                title: 'Event added',
                status: 'success',
                duration: 2000,
                isClosable: true,
              });
            }}
          />
        )}
      </FormControl>

      <FormControl>
        <FormLabel>Tags</FormLabel>
        <CreatableSelect
          isMulti
          options={tagOptions}
          value={formData.selectedTags}
          onChange={(selected: any) => onChange({
            tags: selected ? selected.map((tag: SelectOption) => tag.value) : []
          })}
          placeholder="Select or create tags..."
          styles={selectStyles}
          isClearable
          isDisabled={readOnly}
          formatCreateLabel={(inputValue) => `Create tag "${inputValue}"`}
          onCreateOption={async (inputValue) => {
            if (readOnly) return;

            try {
              const newTag = await tagApi.createTag(inputValue);
              const newOption = { value: newTag.name, label: newTag.name };
              setTagOptions([...tagOptions, newOption]);

              // Update the selected tags
              const updatedTags = [...formData.selectedTags, newOption];
              onChange({
                tags: updatedTags.map(tag => tag.value)
              });

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
          value={formData.selectedPeople}
          onChange={(selected: any) => onChange({
            people: selected ? selected.map((person: SelectOption) => person.value) : []
          })}
          placeholder="Select or create people..."
          styles={selectStyles}
          isClearable
          isDisabled={readOnly}
          formatCreateLabel={(inputValue) => `Create person "${inputValue}"`}
          onCreateOption={async (inputValue) => {
            if (readOnly) return;

            try {
              const newPerson = await personApi.createPerson(inputValue);
              const newOption = { value: newPerson.name, label: newPerson.name };
              setPeopleOptions([...peopleOptions, newOption]);

              // Update the selected people
              const updatedPeople = [...formData.selectedPeople, newOption];
              onChange({
                people: updatedPeople.map(person => person.value)
              });

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

      <FormControl>
        <FormLabel>Shoeboxes</FormLabel>
        <CreatableSelect
          isMulti
          options={shoeboxOptions}
          value={formData.selectedShoeboxes}
          onChange={(selected: any) => onChange({
            shoeboxes: selected ? selected.map((shoebox: SelectOption) => shoebox.value) : []
          })}
          placeholder="Select or create shoeboxes..."
          styles={selectStyles}
          isClearable
          isDisabled={readOnly}
          formatCreateLabel={(inputValue) => `Create shoebox "${inputValue}"`}
          onCreateOption={async (inputValue) => {
            if (readOnly) return;

            try {
              const newShoebox = await shoeboxApi.createShoebox(inputValue);
              const newOption = { value: newShoebox.name, label: newShoebox.name };
              setShoeboxOptions([...shoeboxOptions, newOption]);

              // Update the selected shoeboxes
              const updatedShoeboxes = [...formData.selectedShoeboxes, newOption];
              onChange({
                shoeboxes: updatedShoeboxes.map(shoebox => shoebox.value)
              });

              toast({
                title: 'Shoebox created',
                status: 'success',
                duration: 2000,
                isClosable: true,
              });
            } catch (error) {
              console.error('Error creating shoebox:', error);
              toast({
                title: 'Error creating shoebox',
                status: 'error',
                duration: 3000,
                isClosable: true,
              });
            }
          }}
        />
      </FormControl>
    </>
  );
};

export default VideoForm;
