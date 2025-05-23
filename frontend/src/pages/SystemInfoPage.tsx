import React, { useState, useEffect } from 'react';
import {
  Box,
  Heading,
  Text,
  VStack,
  HStack,
  Divider,
  Card,
  CardHeader,
  CardBody,
  SimpleGrid,
  Spinner,
  Alert,
  AlertIcon,
  AlertTitle,
  AlertDescription,
  Button,
  useToast,
} from '@chakra-ui/react';
import { FaSync } from 'react-icons/fa';

interface MediaPathConfig {
  path: string;
  original_path?: string;
  original_extension?: string;
}

interface SystemConfig {
  server: {
    host: string;
    port: number;
  };
  database: {
    url: string;
    max_connections: number;
  };
  media: {
    source_paths: MediaPathConfig[];
    export_base_path: string;
    thumbnail_path: string;
  };
}

const SystemInfoPage: React.FC = () => {
  const [config, setConfig] = useState<SystemConfig | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [rescanLoading, setRescanLoading] = useState<boolean>(false);
  const toast = useToast();

  useEffect(() => {
    const fetchSystemInfo = async () => {
      try {
        const response = await fetch('/api/system');
        if (!response.ok) {
          throw new Error(`Error fetching system info: ${response.statusText}`);
        }
        const data = await response.json();
        setConfig(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'An unknown error occurred');
      } finally {
        setLoading(false);
      }
    };

    fetchSystemInfo();
  }, []);

  const handleRescan = async () => {
    setRescanLoading(true);
    try {
      const response = await fetch('/api/scan', {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error(`Error rescanning library: ${response.statusText}`);
      }

      const data = await response.json();

      toast({
        title: 'Library Rescanned',
        description: `Successfully rescanned library. Found ${data.new_videos_count} new videos.`,
        status: 'success',
        duration: 5000,
        isClosable: true,
      });
    } catch (err) {
      toast({
        title: 'Rescan Failed',
        description: err instanceof Error ? err.message : 'An unknown error occurred',
        status: 'error',
        duration: 5000,
        isClosable: true,
      });
    } finally {
      setRescanLoading(false);
    }
  };

  if (loading) {
    return (
      <Box textAlign="center" py={10}>
        <Spinner size="xl" />
        <Text mt={4}>Loading system information...</Text>
      </Box>
    );
  }

  if (error) {
    return (
      <Alert status="error" variant="solid" flexDirection="column" alignItems="center" justifyContent="center" textAlign="center" height="200px">
        <AlertIcon boxSize="40px" mr={0} />
        <AlertTitle mt={4} mb={1} fontSize="lg">
          Error Loading System Information
        </AlertTitle>
        <AlertDescription maxWidth="sm">{error}</AlertDescription>
      </Alert>
    );
  }

  return (
    <Box maxW="1200px" mx="auto" p={5}>
      <Heading as="h1" size="xl" mb={6}>
        System Information
      </Heading>

      <SimpleGrid columns={{ base: 1, md: 2, lg: 3 }} spacing={10} mb={8}>
        {/* Server Configuration */}
        <Card>
          <CardHeader>
            <Heading size="md">Server Configuration</Heading>
          </CardHeader>
          <CardBody>
            <VStack align="stretch" spacing={3}>
              <HStack justify="space-between">
                <Text fontWeight="bold">Host:</Text>
                <Text>{config?.server.host}</Text>
              </HStack>
              <HStack justify="space-between">
                <Text fontWeight="bold">Port:</Text>
                <Text>{config?.server.port}</Text>
              </HStack>
            </VStack>
          </CardBody>
        </Card>

        {/* Database Configuration */}
        <Card>
          <CardHeader>
            <Heading size="md">Database Configuration</Heading>
          </CardHeader>
          <CardBody>
            <VStack align="stretch" spacing={3}>
              <HStack justify="space-between">
                <Text fontWeight="bold">URL:</Text>
                <Text isTruncated maxW="200px" title={config?.database.url}>
                  {config?.database.url}
                </Text>
              </HStack>
              <HStack justify="space-between">
                <Text fontWeight="bold">Max Connections:</Text>
                <Text>{config?.database.max_connections}</Text>
              </HStack>
            </VStack>
          </CardBody>
        </Card>

        {/* Media Configuration */}
        <Card>
          <CardHeader>
            <Heading size="md">Media Configuration</Heading>
          </CardHeader>
          <CardBody>
            <VStack align="stretch" spacing={3}>
              <Box>
                <Text fontWeight="bold" mb={1}>Source Paths:</Text>
                {config?.media.source_paths.map((pathConfig, index) => (
                  <Text key={index} fontSize="sm" isTruncated title={pathConfig.path}>
                    {pathConfig.path}
                  </Text>
                ))}
              </Box>
              <Divider />
              <Box>
                <Text fontWeight="bold" mb={1}>Export Base Path:</Text>
                <Text fontSize="sm" isTruncated title={config?.media.export_base_path}>
                  {config?.media.export_base_path}
                </Text>
              </Box>
              <Divider />
              <Box>
                <Text fontWeight="bold" mb={1}>Thumbnail Path:</Text>
                <Text fontSize="sm" isTruncated title={config?.media.thumbnail_path}>
                  {config?.media.thumbnail_path}
                </Text>
              </Box>
            </VStack>
          </CardBody>
        </Card>
      </SimpleGrid>

      <Box textAlign="center" mt={6}>
        <Button
          colorScheme="blue"
          size="lg"
          onClick={handleRescan}
          isLoading={rescanLoading}
          loadingText="Rescanning..."
          leftIcon={<FaSync />}
        >
          Rescan Library
        </Button>
        <Text mt={2} fontSize="sm" color="gray.600">
          Rescans existing library to capture any new metadata or fields that have been added
        </Text>
      </Box>
    </Box>
  );
};

export default SystemInfoPage;
