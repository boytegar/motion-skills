import React from 'react';
import {AbsoluteFill} from 'remotion';

export const SimpleCut: React.FC<{children: React.ReactNode}> = ({children}) => (
  <AbsoluteFill>{children}</AbsoluteFill>
);
