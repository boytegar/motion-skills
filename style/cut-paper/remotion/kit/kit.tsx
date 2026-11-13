import React from 'react';
import {C, FONT} from './tokens';

export const CutPaperDefs: React.FC = () => (
  <svg width={0} height={0} style={{position: 'absolute'}}>
    <defs>
      <filter id="torn-edge">
        <feTurbulence type="fractalNoise" baseFrequency="0.08" numOctaves={3} result="noise" />
        <feDisplacementMap in="SourceGraphic" in2="noise" scale={5} xChannelSelector="R" yChannelSelector="G" />
      </filter>
    </defs>
  </svg>
);

export const CutPaperSlab: React.FC<{
  x: number; y: number; w: number; h: number; color?: string; rot?: number; children?: React.ReactNode;
}> = ({x, y, w, h, color = C.cream, rot = 0, children}) => (
  <div style={{
    position: 'absolute', left: x, top: y, width: w, height: h,
    backgroundColor: color,
    filter: 'url(#torn-edge) drop-shadow(4px 8px 12px rgba(0,0,0,0.28))',
    transform: `rotate(${rot}deg)`,
    borderRadius: 6
  }}>
    {children}
  </div>
);

export const SparkHero: React.FC<{x: number; y: number; r?: number}> = ({x, y, r = 36}) => (
  <div style={{
    position: 'absolute', left: x - r, top: y - r, width: r * 2, height: r * 2,
    backgroundColor: C.spark, borderRadius: '50%',
    filter: 'drop-shadow(2px 4px 6px rgba(0,0,0,0.3))'
  }}>
    <div style={{position: 'absolute', left: r * 0.4, top: r * 0.5, width: 6, height: 6, borderRadius: '50%', background: C.ink}} />
    <div style={{position: 'absolute', right: r * 0.4, top: r * 0.5, width: 6, height: 6, borderRadius: '50%', background: C.ink}} />
  </div>
);
