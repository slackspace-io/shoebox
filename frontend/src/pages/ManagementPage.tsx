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
  useColorModeValue
} from '@chakra-ui/react';
import { FaTrash, FaPlus, FaArrowLeft } from 'react-icons/fa';
import { useNavigate } from 'react-router-dom';
import { tagApi, personApi, TagUsage, PersonUsage } from '../api/client';

const ManagementPage: React.FC = () => {
  const navigate = useNavigate();
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();
  const cancelRef = React.useRef<HTMLButtonElement>(null);

  const [tags, setTags] = useState<TagUsage[]>([]);
  const [people, setPeople] = useState<PersonUsage[]>([]);
  const [loading, setLoading] = useState(true);
  const [newTagName, setNewTagName] = useState('');
  const [newPersonName, setNewPersonName] = useState('');
  const [itemToDelete, setItemToDelete] = useState<{ id: string; name: string; type: 'tag' | 'person' } | null>(null);

  const bgColor = useColorModeValue('white', 'gray.800');

  // Load tags and people
  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const [tagsData, peopleData] = await Promise.all([
          tagApi.getTagUsage(),
          personApi.getPersonUsage()
        ]);
        setTags(tagsData);
        setPeople(peopleData);
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

  // Handle deleting a tag or person
  const handleDelete = async () => {
    if (!itemToDelete) return;

    try {
      if (itemToDelete.type === 'tag') {
        await tagApi.deleteTag(itemToDelete.id);
        setTags(tags.filter(tag => tag.id !== itemToDelete.id));
        toast({
          title: 'Tag deleted',
          status: 'success',
          duration: 2000,
          isClosable: true,
        });
      } else {
        await personApi.deletePerson(itemToDelete.id);
        setPeople(people.filter(person => person.id !== itemToDelete.id));
        toast({
          title: 'Person deleted',
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
  const openDeleteDialog = (id: string, name: string, type: 'tag' | 'person') => {
    setItemToDelete({ id, name, type });
    onOpen();
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
        <Heading size="lg">Manage Tags & People</Heading>
        <Box width="100px" /> {/* Spacer for alignment */}
      </Flex>

      <Tabs isFitted variant="enclosed">
        <TabList mb="1em">
          <Tab>Tags</Tab>
          <Tab>People</Tab>
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
              Delete {itemToDelete?.type === 'tag' ? 'Tag' : 'Person'}
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
