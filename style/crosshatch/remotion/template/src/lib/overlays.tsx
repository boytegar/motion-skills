import React from 'react';
import {AbsoluteFill, useCurrentFrame, random} from 'remotion';
import {C, FONT} from './tokens';

export const CrosshatchBanner: React.FC<{text: string}> = ({text}) => {
  const f = useCurrentFrame();
  const step = Math.floor(f / 2);
  const r = (random(`ban_${step}`) - 0.5) * 1.2;
  return (
    <div style={{
      position: 'absolute', left: 80, top: 220, width: 920,
      background: C.paper, border: `3px solid ${C.ink}`,
      padding: '16px 24px', fontFamily: FONT.banner,
      fontSize: 42, color: C.ink, textTransform: 'uppercase',
      transform: `rotate(${r}deg)`, textAlign: 'center',
      boxShadow: '4px 6px 0 rgba(58,36,23,0.3)'
    }}>
      {text}
    </div>
  );
};

export const CrosshatchCaptions: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 100, top: 1400, width: 880,
    background: C.ink, color: C.paper,
    padding: '12px 20px', fontFamily: FONT.caption,
    fontSize: 34, textAlign: 'center', borderRadius: 4
  }}>
    {text}
  </div>
);

export const GrainOverlay: React.FC = () => (
  <AbsoluteFill style={{
    pointerEvents: 'none', opacity: 0.12,
    backgroundImage: 'radial-gradient(#3a2417 1px, transparent 1px)',
    backgroundSize: '8px 8px'
  }} />
);
