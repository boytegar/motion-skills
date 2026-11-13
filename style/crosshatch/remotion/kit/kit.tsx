import React from 'react';
import {AbsoluteFill, useCurrentFrame, random} from 'remotion';
import {C, FONT} from './tokens';

export const CrosshatchDefs: React.FC = () => (
  <svg width={0} height={0} style={{position: 'absolute'}}>
    <defs>
      <pattern id="hatch-45" width="12" height="12" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
        <line x1="0" y1="0" x2="0" y2="12" stroke={C.ink} strokeWidth="1.5" strokeOpacity="0.4" />
      </pattern>
      <pattern id="cross-45" width="12" height="12" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
        <line x1="0" y1="0" x2="0" y2="12" stroke={C.ink} strokeWidth="1.5" strokeOpacity="0.35" />
        <line x1="0" y1="0" x2="12" y2="0" stroke={C.ink} strokeWidth="1.5" strokeOpacity="0.35" />
      </pattern>
      <filter id="ink-boil">
        <feTurbulence type="fractalNoise" baseFrequency="0.04" numOctaves={3} result="noise" />
        <feDisplacementMap in="SourceGraphic" in2="noise" scale={3.5} xChannelSelector="R" yChannelSelector="G" />
      </filter>
    </defs>
  </svg>
);

export const CrosshatchSlab: React.FC<{
  x: number; y: number; w: number; h: number;
  bg?: string; hatch?: boolean; cross?: boolean; children?: React.ReactNode;
}> = ({x, y, w, h, bg = C.cream, hatch = true, cross = false, children}) => {
  const f = useCurrentFrame();
  const step = Math.floor(f / 2);
  const wobble = (random(`ch_w_${step}`) - 0.5) * 1.5;
  return (
    <div style={{
      position: 'absolute', left: x, top: y, width: w, height: h,
      backgroundColor: bg,
      border: `2.5px solid ${C.ink}`,
      filter: 'url(#ink-boil)',
      transform: `rotate(${wobble}deg)`,
      overflow: 'hidden'
    }}>
      {hatch && (
        <svg style={{position: 'absolute', inset: 0, width: '100%', height: '100%'}}>
          <rect width="100%" height="100%" fill={`url(#${cross ? 'cross-45' : 'hatch-45'})`} />
        </svg>
      )}
      {children}
    </div>
  );
};

export const ToolPill: React.FC<{
  x: number; y: number; tool: string; arg: string;
}> = ({x, y, tool, arg}) => (
  <div style={{
    position: 'absolute', left: x, top: y,
    display: 'flex', alignItems: 'center', gap: 10,
    background: C.navy, color: C.peri,
    border: `2px solid ${C.peri}`,
    padding: '8px 16px', borderRadius: 8,
    fontFamily: FONT.caption, fontSize: 24
  }}>
    <span style={{color: C.glow, fontWeight: 700}}>{tool}</span>
    <span>{arg}</span>
  </div>
);
