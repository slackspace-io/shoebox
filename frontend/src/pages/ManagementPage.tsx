import React, { useState, useEffect } from 'react';
import {
  Box,
  Heading,
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  VStack,
  HStack,
  Button,
  Input,
  FormControl,
  useToast,
  Spinner,
  Badge,
  AlertDialog,
  AlertDialogBody,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogContent,
  AlertDialogOverlay,
  useDisclosure,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  IconButton,
  Flex,
  Text,
  useColorModeValue
} from '@chakra-ui/react';
import { FaTrash, FaPlus, FaArrowLeft, FaEdit, FaSave } from 'react-icons/fa';
import { useNavigate } from 'react-router-dom';
import { tagApi, personApi, locationApi, eventApi, TagUsage, PersonUsage, LocationUsage, EventUsage } from '../api/client';

const ManagementPage: React.FC = () => {
  const navigate = useNavigate();
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();
  const cancelRef = React.useRef<HTMLButtonElement>(null);

  const [tags, setTags] = useState<TagUsage[]>([]);
  const [people, setPeople] = useState<PersonUsage[]>([]);
  const [locations, setLocations] = useState<LocationUsage[]>([]);
  const [events, setEvents] = useState<EventUsage[]>([]);
  const [loading, setLoading] = useState(true);
  const [newTagName, setNewTagName] = useState('');
  const [newPersonName, setNewPersonName] = useState('');
  const [editingLocation, setEditingLocation] = useState<{ oldName: string; newName: string } | null>(null);
  const [editingEvent, setEditingEvent] = useState<{ oldName: string; newName: string } | null>(null);
  const [itemToDelete, setItemToDelete] = useState<{ id?: string; name: string; type: 'tag' | 'person' | 'location' | 'event' } | null>(null);

  const bgColor = useColorModeValue('white', 'gray.800');

  // Load tags, people, locations, and events
  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const [tagsData, peopleData, locationsData, eventsData] = await Promise.all([
          tagApi.getTagUsage(),
          personApi.getPersonUsage(),
          locationApi.getLocationUsage(),
          eventApi.getEventUsage()
        ]);
        setTags(tagsData);
        setPeople(peopleData);
        setLocations(locationsData);
        setEvents(eventsData);
      } catch (error) {
        console.error('Error fetching data:', error);
        toast({
          title: 'Error fetching data',
          status: 'error',
          duration: 3000,
          isClosable: true,
        });
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [toast]);

  // Handle adding a new tag
  const handleAddTag = async () => {
    if (!newTagName.trim()) {
      toast({
        title: 'Tag name cannot be empty',
        status: 'warning',
        duration: 2000,
        isClosable: true,
      });
      return;
    }

    try {
      const newTag = await tagApi.createTag(newTagName);
      setTags([...tags, { id: newTag.id, name: newTag.name, video_count: 0 }]);
      setNewTagName('');
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
  };

  // Handle adding a new person
  const handleAddPerson = async () => {
    if (!newPersonName.trim()) {
      toast({
        title: 'Person name cannot be empty',
        status: 'warning',
        duration: 2000,
        isClosable: true,
      });
      return;
    }

    try {
      const newPerson = await personApi.createPerson(newPersonName);
      setPeople([...people, { id: newPerson.id, name: newPerson.name, video_count: 0 }]);
      setNewPersonName('');
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
  };

  // Handle deleting a tag, person, location, or event
  const handleDelete = async () => {
    if (!itemToDelete) return;

    try {
      if (itemToDelete.type === 'tag' && itemToDelete.id) {
        await tagApi.deleteTag(itemToDelete.id);
        setTags(tags.filter(tag => tag.id !== itemToDelete.id));
        toast({
          title: 'Tag deleted',
          status: 'success',
          duration: 2000,
          isClosable: true,
        });
      } else if (itemToDelete.type === 'person' && itemToDelete.id) {
        await personApi.deletePerson(itemToDelete.id);
        setPeople(people.filter(person => person.id !== itemToDelete.id));
        toast({
          title: 'Person deleted',
          status: 'success',
          duration: 2000,
          isClosable: true,
        });
      } else if (itemToDelete.type === 'location') {
        const count = await locationApi.deleteLocation(itemToDelete.name);
        // Refresh locations after deletion
        const updatedLocations = await locationApi.getLocationUsage();
        setLocations(updatedLocations);
        toast({
          title: `Location deleted from ${count} videos`,
          status: 'success',
          duration: 2000,
          isClosable: true,
        });
      } else if (itemToDelete.type === 'event') {
        const count = await eventApi.deleteEvent(itemToDelete.name);
        // Refresh events after deletion
        const updatedEvents = await eventApi.getEventUsage();
        setEvents(updatedEvents);
        toast({
          title: `Event deleted from ${count} videos`,
          status: 'success',
          duration: 2000,
          isClosable: true,
        });
      }
    } catch (error) {
      console.error('Error deleting item:', error);
      toast({
        title: 'Error deleting item',
        description: 'The item may be in use by videos.',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    } finally {
      onClose();
      setItemToDelete(null);
    }
  };

  // Open delete confirmation dialog
  const openDeleteDialog = (id: string | undefined, name: string, type: 'tag' | 'person' | 'location' | 'event') => {
    setItemToDelete({ id, name, type });
    onOpen();
  };

  // Handle updating a location
  const handleUpdateLocation = async () => {
    if (!editingLocation) return;

    try {
      const count = await locationApi.updateLocation(editingLocation.oldName, editingLocation.newName);
      // Refresh locations after update
      const updatedLocations = await locationApi.getLocationUsage();
      setLocations(updatedLocations);
      setEditingLocation(null);
      toast({
        title: `Location updated in ${count} videos`,
        status: 'success',
        duration: 2000,
        isClosable: true,
      });
    } catch (error) {
      console.error('Error updating location:', error);
      toast({
        title: 'Error updating location',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    }
  };

  // Handle updating an event
  const handleUpdateEvent = async () => {
    if (!editingEvent) return;

    try {
      const count = await eventApi.updateEvent(editingEvent.oldName, editingEvent.newName);
      // Refresh events after update
      const updatedEvents = await eventApi.getEventUsage();
      setEvents(updatedEvents);
      setEditingEvent(null);
      toast({
        title: `Event updated in ${count} videos`,
        status: 'success',
        duration: 2000,
        isClosable: true,
      });
    } catch (error) {
      console.error('Error updating event:', error);
      toast({
        title: 'Error updating event',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    }
  };

  // Handle deleting all unused tags
  const handleDeleteUnusedTags = async () => {
    try {
      await tagApi.deleteUnusedTags();
      const updatedTags = await tagApi.getTagUsage();
      setTags(updatedTags);
      toast({
        title: 'Unused tags deleted',
        status: 'success',
        duration: 2000,
        isClosable: true,
      });
    } catch (error) {
      console.error('Error deleting unused tags:', error);
      toast({
        title: 'Error deleting unused tags',
        status: 'error',
        duration: 3000,
        isClosable: true,
      });
    }
  };

  // Handle deleting all unused people
  const handleDeleteUnusedPeople = async () => {
    try {
      await personApi.deleteUnusedPeople();
      const updatedPeople = await personApi.getPersonUsage();
      setPeople(updatedPeople);
      toast({
        title: 'Unused people deleted',
        status: 'success',
        duration: 2000,
        isClosable: true,
      });
    } catch (error) {
      console.error('Error deleting unused people:', error);
      toast({
        title: 'Error deleting unused people',
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

  return (
    <Box>
      <Flex mb={6} justify="space-between" align="center">
        <Button leftIcon={<FaArrowLeft />} onClick={() => navigate('/')}>
          Back to Videos
        </Button>
        <Heading size="lg">Manage Data</Heading>
        <Box width="100px" /> {/* Spacer for alignment */}
      </Flex>

      <Tabs isFitted variant="enclosed">
        <TabList mb="1em">
          <Tab>Tags</Tab>
          <Tab>People</Tab>
          <Tab>Locations</Tab>
          <Tab>Events</Tab>
        </TabList>

        <TabPanels>
          {/* Tags Panel */}
          <TabPanel>
            <VStack spacing={4} align="stretch">
              <Box p={4} borderWidth="1px" borderRadius="md" bg={bgColor}>
                <Heading size="md" mb={4}>Add New Tag</Heading>
                <HStack>
                  <FormControl>
                    <Input
                      value={newTagName}
                      onChange={(e) => setNewTagName(e.target.value)}
                      placeholder="Enter tag name"
                    />
                  </FormControl>
                  <Button
                    leftIcon={<FaPlus />}
                    colorScheme="blue"
                    onClick={handleAddTag}
                  >
                    Add
                  </Button>
                </HStack>
              </Box>

              <Box p={4} borderWidth="1px" borderRadius="md" bg={bgColor}>
                <Flex justify="space-between" align="center" mb={4}>
                  <Heading size="md">Existing Tags</Heading>
                  <Button
                    size="sm"
                    colorScheme="red"
                    variant="outline"
                    onClick={handleDeleteUnusedTags}
                  >
                    Delete All Unused
                  </Button>
                </Flex>
                <Table variant="simple">
                  <Thead>
                    <Tr>
                      <Th>Name</Th>
                      <Th isNumeric>Videos</Th>
                      <Th width="80px">Actions</Th>
                    </Tr>
                  </Thead>
                  <Tbody>
                    {tags.length > 0 ? (
                      tags.map((tag) => (
                        <Tr key={tag.id}>
                          <Td>
                            <Badge colorScheme="blue" color="white">{tag.name}</Badge>
                          </Td>
                          <Td isNumeric>{tag.video_count}</Td>
                          <Td>
                            <IconButton
                              aria-label="Delete tag"
                              icon={<FaTrash />}
                              size="sm"
                              colorScheme="red"
                              variant="ghost"
                              isDisabled={tag.video_count > 0}
                              onClick={() => openDeleteDialog(tag.id, tag.name, 'tag')}
                            />
                          </Td>
                        </Tr>
                      ))
                    ) : (
                      <Tr>
                        <Td colSpan={3} textAlign="center">No tags found</Td>
                      </Tr>
                    )}
                  </Tbody>
                </Table>
              </Box>
            </VStack>
          </TabPanel>

          {/* People Panel */}
          <TabPanel>
            <VStack spacing={4} align="stretch">
              <Box p={4} borderWidth="1px" borderRadius="md" bg={bgColor}>
                <Heading size="md" mb={4}>Add New Person</Heading>
                <HStack>
                  <FormControl>
                    <Input
                      value={newPersonName}
                      onChange={(e) => setNewPersonName(e.target.value)}
                      placeholder="Enter person name"
                    />
                  </FormControl>
                  <Button
                    leftIcon={<FaPlus />}
                    colorScheme="green"
                    onClick={handleAddPerson}
                  >
                    Add
                  </Button>
                </HStack>
              </Box>

              <Box p={4} borderWidth="1px" borderRadius="md" bg={bgColor}>
                <Flex justify="space-between" align="center" mb={4}>
                  <Heading size="md">Existing People</Heading>
                  <Button
                    size="sm"
                    colorScheme="red"
                    variant="outline"
                    onClick={handleDeleteUnusedPeople}
                  >
                    Delete All Unused
                  </Button>
                </Flex>
                <Table variant="simple">
                  <Thead>
                    <Tr>
                      <Th>Name</Th>
                      <Th isNumeric>Videos</Th>
                      <Th width="80px">Actions</Th>
                    </Tr>
                  </Thead>
                  <Tbody>
                    {people.length > 0 ? (
                      people.map((person) => (
                        <Tr key={person.id}>
                          <Td>
                            <Badge colorScheme="green" color="white">{person.name}</Badge>
                          </Td>
                          <Td isNumeric>{person.video_count}</Td>
                          <Td>
                            <IconButton
                              aria-label="Delete person"
                              icon={<FaTrash />}
                              size="sm"
                              colorScheme="red"
                              variant="ghost"
                              isDisabled={person.video_count > 0}
                              onClick={() => openDeleteDialog(person.id, person.name, 'person')}
                            />
                          </Td>
                        </Tr>
                      ))
                    ) : (
                      <Tr>
                        <Td colSpan={3} textAlign="center">No people found</Td>
                      </Tr>
                    )}
                  </Tbody>
                </Table>
              </Box>
            </VStack>
          </TabPanel>

          {/* Locations Panel */}
          <TabPanel>
            <VStack spacing={4} align="stretch">
              <Box p={4} borderWidth="1px" borderRadius="md" bg={bgColor}>
                <Heading size="md" mb={4}>Manage Locations</Heading>
                <Text mb={4}>
                  Locations are automatically created when you add them to videos.
                  Here you can rename or delete existing locations.
                </Text>
              </Box>

              <Box p={4} borderWidth="1px" borderRadius="md" bg={bgColor}>
                <Heading size="md" mb={4}>Existing Locations</Heading>
                <Table variant="simple">
                  <Thead>
                    <Tr>
                      <Th>Name</Th>
                      <Th isNumeric>Videos</Th>
                      <Th width="120px">Actions</Th>
                    </Tr>
                  </Thead>
                  <Tbody>
                    {locations.length > 0 ? (
                      locations.map((location) => (
                        <Tr key={location.name}>
                          <Td>
                            {editingLocation && editingLocation.oldName === location.name ? (
                              <Input
                                value={editingLocation.newName}
                                onChange={(e) => setEditingLocation({
                                  ...editingLocation,
                                  newName: e.target.value
                                })}
                                size="sm"
                              />
                            ) : (
                              <Badge colorScheme="purple" color="white">{location.name}</Badge>
                            )}
                          </Td>
                          <Td isNumeric>{location.video_count}</Td>
                          <Td>
                            {editingLocation && editingLocation.oldName === location.name ? (
                              <HStack spacing={1}>
                                <IconButton
                                  aria-label="Save location"
                                  icon={<FaSave />}
                                  size="sm"
                                  colorScheme="green"
                                  variant="ghost"
                                  onClick={handleUpdateLocation}
                                />
                                <IconButton
                                  aria-label="Cancel"
                                  icon={<FaArrowLeft />}
                                  size="sm"
                                  colorScheme="gray"
                                  variant="ghost"
                                  onClick={() => setEditingLocation(null)}
                                />
                              </HStack>
                            ) : (
                              <HStack spacing={1}>
                                <IconButton
                                  aria-label="Edit location"
                                  icon={<FaEdit />}
                                  size="sm"
                                  colorScheme="blue"
                                  variant="ghost"
                                  onClick={() => setEditingLocation({
                                    oldName: location.name,
                                    newName: location.name
                                  })}
                                />
                                <IconButton
                                  aria-label="Delete location"
                                  icon={<FaTrash />}
                                  size="sm"
                                  colorScheme="red"
                                  variant="ghost"
                                  onClick={() => openDeleteDialog(undefined, location.name, 'location')}
                                />
                              </HStack>
                            )}
                          </Td>
                        </Tr>
                      ))
                    ) : (
                      <Tr>
                        <Td colSpan={3} textAlign="center">No locations found</Td>
                      </Tr>
                    )}
                  </Tbody>
                </Table>
              </Box>
            </VStack>
          </TabPanel>

          {/* Events Panel */}
          <TabPanel>
            <VStack spacing={4} align="stretch">
              <Box p={4} borderWidth="1px" borderRadius="md" bg={bgColor}>
                <Heading size="md" mb={4}>Manage Events</Heading>
                <Text mb={4}>
                  Events are automatically created when you add them to videos.
                  Here you can rename or delete existing events.
                </Text>
              </Box>

              <Box p={4} borderWidth="1px" borderRadius="md" bg={bgColor}>
                <Heading size="md" mb={4}>Existing Events</Heading>
                <Table variant="simple">
                  <Thead>
                    <Tr>
                      <Th>Name</Th>
                      <Th isNumeric>Videos</Th>
                      <Th width="120px">Actions</Th>
                    </Tr>
                  </Thead>
                  <Tbody>
                    {events.length > 0 ? (
                      events.map((event) => (
                        <Tr key={event.name}>
                          <Td>
                            {editingEvent && editingEvent.oldName === event.name ? (
                              <Input
                                value={editingEvent.newName}
                                onChange={(e) => setEditingEvent({
                                  ...editingEvent,
                                  newName: e.target.value
                                })}
                                size="sm"
                              />
                            ) : (
                              <Badge colorScheme="orange" color="white">{event.name}</Badge>
                            )}
                          </Td>
                          <Td isNumeric>{event.video_count}</Td>
                          <Td>
                            {editingEvent && editingEvent.oldName === event.name ? (
                              <HStack spacing={1}>
                                <IconButton
                                  aria-label="Save event"
                                  icon={<FaSave />}
                                  size="sm"
                                  colorScheme="green"
                                  variant="ghost"
                                  onClick={handleUpdateEvent}
                                />
                                <IconButton
                                  aria-label="Cancel"
                                  icon={<FaArrowLeft />}
                                  size="sm"
                                  colorScheme="gray"
                                  variant="ghost"
                                  onClick={() => setEditingEvent(null)}
                                />
                              </HStack>
                            ) : (
                              <HStack spacing={1}>
                                <IconButton
                                  aria-label="Edit event"
                                  icon={<FaEdit />}
                                  size="sm"
                                  colorScheme="blue"
                                  variant="ghost"
                                  onClick={() => setEditingEvent({
                                    oldName: event.name,
                                    newName: event.name
                                  })}
                                />
                                <IconButton
                                  aria-label="Delete event"
                                  icon={<FaTrash />}
                                  size="sm"
                                  colorScheme="red"
                                  variant="ghost"
                                  onClick={() => openDeleteDialog(undefined, event.name, 'event')}
                                />
                              </HStack>
                            )}
                          </Td>
                        </Tr>
                      ))
                    ) : (
                      <Tr>
                        <Td colSpan={3} textAlign="center">No events found</Td>
                      </Tr>
                    )}
                  </Tbody>
                </Table>
              </Box>
            </VStack>
          </TabPanel>
        </TabPanels>
      </Tabs>

      {/* Delete confirmation dialog */}
      <AlertDialog
        isOpen={isOpen}
        leastDestructiveRef={cancelRef}
        onClose={onClose}
      >
        <AlertDialogOverlay>
          <AlertDialogContent>
            <AlertDialogHeader fontSize="lg" fontWeight="bold">
              Delete {
                itemToDelete?.type === 'tag' ? 'Tag' :
                itemToDelete?.type === 'person' ? 'Person' :
                itemToDelete?.type === 'location' ? 'Location' : 'Event'
              }
            </AlertDialogHeader>

            <AlertDialogBody>
              Are you sure you want to delete "{itemToDelete?.name}"? This action cannot be undone.
            </AlertDialogBody>

            <AlertDialogFooter>
              <Button ref={cancelRef} onClick={onClose}>
                Cancel
              </Button>
              <Button colorScheme="red" onClick={handleDelete} ml={3}>
                Delete
              </Button>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialogOverlay>
      </AlertDialog>
    </Box>
  );
};

export default ManagementPage;
