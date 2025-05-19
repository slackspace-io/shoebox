import React from 'react';
import { Box, Flex, Heading, Link, Spacer, Button, useColorMode, useColorModeValue } from '@chakra-ui/react';
import { Link as RouterLink, useLocation } from 'react-router-dom';
import { FaSun, FaMoon, FaVideo, FaFileExport } from 'react-icons/fa';

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const { colorMode, toggleColorMode } = useColorMode();
  const location = useLocation();
  const bgColor = useColorModeValue('white', 'gray.800');
  const borderColor = useColorModeValue('gray.200', 'gray.700');

  return (
    <Box>
      <Flex
        as="header"
        align="center"
        justify="space-between"
        wrap="wrap"
        padding={4}
        bg={bgColor}
        borderBottom="1px"
        borderColor={borderColor}
        position="sticky"
        top={0}
        zIndex={10}
      >
        <Flex align="center" mr={5}>
          <Heading as="h1" size="lg" letterSpacing="tight">
            <Link as={RouterLink} to="/" _hover={{ textDecoration: 'none' }}>
              Family Video Organizer
            </Link>
          </Heading>
        </Flex>

        <Spacer />

        <Flex align="center">
          <Link
            as={RouterLink}
            to="/"
            mr={4}
            display="flex"
            alignItems="center"
            fontWeight={location.pathname === '/' ? 'bold' : 'normal'}
            color={location.pathname === '/' ? 'brand.500' : undefined}
          >
            <FaVideo style={{ marginRight: '8px' }} />
            Videos
          </Link>
          <Link
            as={RouterLink}
            to="/export"
            mr={4}
            display="flex"
            alignItems="center"
            fontWeight={location.pathname === '/export' ? 'bold' : 'normal'}
            color={location.pathname === '/export' ? 'brand.500' : undefined}
          >
            <FaFileExport style={{ marginRight: '8px' }} />
            Export
          </Link>
          <Button onClick={toggleColorMode} size="sm" ml={4}>
            {colorMode === 'light' ? <FaMoon /> : <FaSun />}
          </Button>
        </Flex>
      </Flex>

      <Box as="main" p={4}>
        {children}
      </Box>
    </Box>
  );
};

export default Layout;
