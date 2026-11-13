import React from 'react';
import {C, FONT} from './tokens';

export const MathGrid: React.FC = () => (
  <svg style={{position: 'absolute', inset: 0, width: '100%', height: '100%', opacity: 0.25}}>
    <defs>
      <pattern id="math-grid" width="40" height="40" patternUnits="userSpaceOnUse">
        <path d="M 40 0 L 0 0 0 40" fill="none" stroke={C.grid} strokeWidth="1" />
      </pattern>
    </defs>
    <rect width="100%" height="100%" fill="url(#math-grid)" />
  </svg>
);

export const MathAxes: React.FC<{cx?: number; cy?: number}> = ({cx = 540, cy = 960}) => (
  <svg style={{position: 'absolute', inset: 0, width: '100%', height: '100%', pointerEvents: 'none'}}>
    <line x1="80" y1={cy} x2="1000" y2={cy} stroke={C.formula} strokeWidth="2.5" />
    <line x1={cx} y1="400" x2={cx} y2="1500" stroke={C.formula} strokeWidth="2.5" />
  </svg>
);

export const Formula: React.FC<{
  x: number; y: number; text: string; color?: string; size?: number;
}> = ({x, y, text, color = C.yellow, size = 48}) => (
  <div style={{
    position: 'absolute', left: x, top: y,
    color, fontFamily: FONT.math, fontStyle: 'italic',
    fontSize: size, letterSpacing: '0.05em'
  }}>
    {text}
  </div>
);
