import React from 'react';
import ReactDOM from 'react-dom/client';
import { ChakraProvider } from '@chakra-ui/react';
import { BrowserRouter as Router } from 'react-router-dom';
import App from './App';
import theme from './theme';
import { ScanProvider } from './contexts/ScanContext';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ChakraProvider theme={theme}>
      <Router>
        <ScanProvider>
          <App />
        </ScanProvider>
      </Router>
    </ChakraProvider>
  </React.StrictMode>
);
