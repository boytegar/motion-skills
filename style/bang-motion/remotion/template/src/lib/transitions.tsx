import React from 'react';
import {AbsoluteFill, interpolate, useCurrentFrame} from 'remotion';

export const LightSweepWipe: React.FC<{
  progress: number;
  children: React.ReactNode;
}> = ({progress, children}) => {
  const p = Math.max(0, Math.min(1, progress));
  return (
    <AbsoluteFill style={{opacity: 1 - p * 0.2}}>
      {children}
    </AbsoluteFill>
  );
};
