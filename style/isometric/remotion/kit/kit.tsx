import React from 'react';
import {C, FONT} from './tokens';

export const IsometricGrid: React.FC = () => (
  <svg style={{position: 'absolute', inset: 0, width: '100%', height: '100%', opacity: 0.15}}>
    <defs>
      <pattern id="iso-grid" width="60" height="34.64" patternUnits="userSpaceOnUse">
        <path d="M 0 17.32 L 60 17.32 M 30 0 L 30 34.64" stroke={C.lines} strokeWidth="1" />
      </pattern>
    </defs>
    <rect width="100%" height="100%" fill="url(#iso-grid)" />
  </svg>
);

export const IsometricBlock: React.FC<{
  x: number; y: number; w: number; h: number; d: number; color?: string;
}> = ({x, y, w, h, d, color = C.fill}) => {
  const cos30 = 0.866;
  const sin30 = 0.5;
  return (
    <div style={{position: 'absolute', left: x, top: y}}>
      <svg width={w * 2 + 50} height={h + d + 50} style={{overflow: 'visible'}}>
        {/* Top face */}
        <polygon
          points={`${w * cos30},0 ${w * cos30 * 2},${w * sin30} ${w * cos30},${w * sin30 * 2} 0,${w * sin30}`}
          fill={color} stroke={C.edge} strokeWidth="2"
        />
        {/* Left face */}
        <polygon
          points={`0,${w * sin30} ${w * cos30},${w * sin30 * 2} ${w * cos30},${w * sin30 * 2 + d} 0,${w * sin30 + d}`}
          fill="#ececf2" stroke={C.edge} strokeWidth="2"
        />
        {/* Right face */}
        <polygon
          points={`${w * cos30},${w * sin30 * 2} ${w * cos30 * 2},${w * sin30} ${w * cos30 * 2},${w * sin30 + d} ${w * cos30},${w * sin30 * 2 + d}`}
          fill="#d8d8e2" stroke={C.edge} strokeWidth="2"
        />
      </svg>
    </div>
  );
};
