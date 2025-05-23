import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface ScanStatus {
  inProgress: boolean;
  newVideosCount: number;
  updatedVideosCount: number;
}

interface ScanContextType {
  scanStatus: ScanStatus;
  checkScanStatus: () => Promise<void>;
}

const defaultScanStatus: ScanStatus = {
  inProgress: false,
  newVideosCount: 0,
  updatedVideosCount: 0,
};

const ScanContext = createContext<ScanContextType | undefined>(undefined);

export const useScanContext = () => {
  const context = useContext(ScanContext);
  if (context === undefined) {
    throw new Error('useScanContext must be used within a ScanProvider');
  }
  return context;
};

interface ScanProviderProps {
  children: ReactNode;
}

export const ScanProvider: React.FC<ScanProviderProps> = ({ children }) => {
  const [scanStatus, setScanStatus] = useState<ScanStatus>(defaultScanStatus);

  const checkScanStatus = async () => {
    try {
      const response = await fetch('/api/scan/status');
      if (!response.ok) {
        throw new Error('Failed to fetch scan status');
      }
      const data = await response.json();
      setScanStatus({
        inProgress: data.in_progress,
        newVideosCount: data.new_videos_count,
        updatedVideosCount: data.updated_videos_count,
      });
    } catch (error) {
      console.error('Error checking scan status:', error);
    }
  };

  // Check scan status on mount and every 5 seconds if a scan is in progress
  useEffect(() => {
    checkScanStatus();

    const intervalId = setInterval(() => {
      if (scanStatus.inProgress) {
        checkScanStatus();
      }
    }, 5000);

    return () => clearInterval(intervalId);
  }, [scanStatus.inProgress]);

  return (
    <ScanContext.Provider value={{ scanStatus, checkScanStatus }}>
      {children}
    </ScanContext.Provider>
  );
};
