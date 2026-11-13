import React from 'react';
import {C, FONT} from './tokens';

export const DotGridPage: React.FC = () => (
  <svg style={{position: 'absolute', inset: 0, width: '100%', height: '100%', opacity: 0.35}}>
    <defs>
      <pattern id="dot-grid" width="30" height="30" patternUnits="userSpaceOnUse">
        <circle cx="15" cy="15" r="1.5" fill={C.line} />
      </pattern>
    </defs>
    <rect width="100%" height="100%" fill="url(#dot-grid)" />
  </svg>
);

export const SketchScribble: React.FC<{
  x: number; y: number; w: number; h: number; color?: string;
}> = ({x, y, w, h, color = C.accent}) => (
  <div style={{
    position: 'absolute', left: x, top: y, width: w, height: h,
    background: color, opacity: 0.45, borderRadius: '40% 60% 70% 30% / 40% 50% 60% 50%',
    filter: 'blur(2px)'
  }} />
);
