import React from 'react';
import { Routes, Route } from 'react-router-dom';
import { Box } from '@chakra-ui/react';
import Layout from './components/Layout';
import HomePage from './pages/HomePage';
import VideoDetailPage from './pages/VideoDetailPage';
import ExportPage from './pages/ExportPage';
import ManagementPage from './pages/ManagementPage';
import UnreviewedPage from './pages/UnreviewedPage';
import SystemInfoPage from './pages/SystemInfoPage';

const App: React.FC = () => {
  return (
    <Box minH="100vh">
      <Layout>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/videos/:id" element={<VideoDetailPage />} />
          <Route path="/unreviewed" element={<UnreviewedPage />} />
          <Route path="/export" element={<ExportPage />} />
          <Route path="/manage" element={<ManagementPage />} />
          <Route path="/system" element={<SystemInfoPage />} />
        </Routes>
      </Layout>
    </Box>
  );
};

export default App;
