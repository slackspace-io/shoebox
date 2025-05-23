import React from 'react';
import { Box, Flex, Heading, Link, Spacer, Button, useColorMode, useColorModeValue, Alert, AlertIcon, AlertTitle, AlertDescription, Spinner } from '@chakra-ui/react';
import { Link as RouterLink, useLocation } from 'react-router-dom';
import { FaSun, FaMoon, FaVideo, FaFileExport, FaTags, FaClipboardCheck, FaCog } from 'react-icons/fa';
import { useScanContext } from '../contexts/ScanContext';

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const { colorMode, toggleColorMode } = useColorMode();
  const location = useLocation();
  const bgColor = useColorModeValue('white', 'gray.800');
  const borderColor = useColorModeValue('gray.200', 'gray.700');
  const { scanStatus } = useScanContext();

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
              Shoebox
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
            to="/unreviewed"
            mr={4}
            display="flex"
            alignItems="center"
            fontWeight={location.pathname === '/unreviewed' ? 'bold' : 'normal'}
            color={location.pathname === '/unreviewed' ? 'brand.500' : undefined}
          >
            <FaClipboardCheck style={{ marginRight: '8px' }} />
            Unreviewed
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
          <Link
            as={RouterLink}
            to="/manage"
            mr={4}
            display="flex"
            alignItems="center"
            fontWeight={location.pathname === '/manage' ? 'bold' : 'normal'}
            color={location.pathname === '/manage' ? 'brand.500' : undefined}
          >
            <FaTags style={{ marginRight: '8px' }} />
            Manage Tags & People
          </Link>
          <Link
            as={RouterLink}
            to="/system"
            mr={4}
            display="flex"
            alignItems="center"
            fontWeight={location.pathname === '/system' ? 'bold' : 'normal'}
            color={location.pathname === '/system' ? 'brand.500' : undefined}
          >
            <FaCog style={{ marginRight: '8px' }} />
            System Info
          </Link>
          <Button onClick={toggleColorMode} size="sm" ml={4}>
            {colorMode === 'light' ? <FaMoon /> : <FaSun />}
          </Button>
        </Flex>
      </Flex>

      {scanStatus.inProgress && (
        <Alert status="info" variant="solid">
          <AlertIcon />
          <Flex align="center">
            <Spinner size="sm" mr={2} />
            <AlertTitle>Scan in progress</AlertTitle>
            <AlertDescription ml={2}>
              {scanStatus.newVideosCount > 0 || scanStatus.updatedVideosCount > 0 ?
                `Found ${scanStatus.newVideosCount} new videos and updated ${scanStatus.updatedVideosCount} videos so far.` :
                'Scanning for videos...'}
            </AlertDescription>
          </Flex>
        </Alert>
      )}

      <Box as="main" p={4}>
        {children}
      </Box>
    </Box>
  );
};

export default Layout;
