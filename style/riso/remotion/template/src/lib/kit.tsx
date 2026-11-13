import React from 'react';
import {C, FONT} from './tokens';

export const RisoPlate: React.FC<{
  ink: string; dx?: number; dy?: number; opacity?: number; children: React.ReactNode;
}> = ({ink, dx = 0, dy = 0, opacity = 0.85, children}) => (
  <div style={{
    position: 'absolute', inset: 0,
    mixBlendMode: 'multiply',
    color: ink,
    opacity,
    transform: `translate(${dx}px, ${dy}px)`
  }}>
    {children}
  </div>
);

export const HalftoneDotFilter: React.FC = () => (
  <svg width={0} height={0} style={{position: 'absolute'}}>
    <defs>
      <pattern id="riso-dots" width="8" height="8" patternUnits="userSpaceOnUse">
        <circle cx="4" cy="4" r="2" fill="currentColor" />
      </pattern>
    </defs>
  </svg>
);
