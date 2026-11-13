import React from 'react';
import {AbsoluteFill, useCurrentFrame} from 'remotion';
import {C, FONT} from './tokens';

export const ParchmentStage: React.FC<{
  timeOfDay?: 'day' | 'dusk' | 'night';
  children?: React.ReactNode;
}> = ({timeOfDay = 'day', children}) => {
  const bg = timeOfDay === 'night' ? C.skyNight : timeOfDay === 'dusk' ? C.skyDusk : C.paper;
  return (
    <AbsoluteFill style={{
      backgroundColor: bg,
      overflow: 'hidden',
      backgroundImage: 'radial-gradient(circle at 50% 30%, rgba(255,255,255,0.4), transparent 80%)'
    }}>
      {children}
    </AbsoluteFill>
  );
};

export const LineArtWalker: React.FC<{
  x: number;
  y: number;
  scale?: number;
  color?: string;
}> = ({x, y, scale = 1.2, color = C.ink}) => {
  const f = useCurrentFrame();
  const stride = Math.sin(f / 4) * 25;
  const leg1 = `M 0 100 L ${stride} 170`;
  const leg2 = `M 0 100 L ${-stride} 170`;
  const arm1 = `M 0 45 L ${-stride * 0.8} 95`;
  const arm2 = `M 0 45 L ${stride * 0.8} 95`;

  return (
    <g transform={`translate(${x}, ${y}) scale(${scale})`}>
      <circle cx="0" cy="15" r="14" fill="none" stroke={color} strokeWidth="2.5" />
      <path d="M -20 8 Q 0 -6 20 8" stroke={color} strokeWidth="2.5" fill="none" />
      <path d="M 0 30 L -12 95 L 12 95 Z" stroke={color} strokeWidth="2" fill="none" />
      <path d={leg1} stroke={color} strokeWidth="2.5" strokeLinecap="round" />
      <path d={leg2} stroke={color} strokeWidth="2.5" strokeLinecap="round" />
      <path d={arm1} stroke={color} strokeWidth="2.2" strokeLinecap="round" />
      <path d={arm2} stroke={color} strokeWidth="2.2" strokeLinecap="round" />
    </g>
  );
};

export const LineArtHouse: React.FC<{
  x: number;
  y: number;
  w?: number;
  h?: number;
  color?: string;
  lit?: boolean;
}> = ({x, y, w = 320, h = 260, color = C.ink, lit = false}) => (
  <g transform={`translate(${x}, ${y})`}>
    <path d={`M -20 0 L ${w / 2} -60 L ${w + 20} 0 Z`} stroke={color} strokeWidth="2.5" fill="none" />
    <rect x="0" y="0" width={w} height={h} stroke={color} strokeWidth="2" fill="none" />
    <rect x={w * 0.15} y={h * 0.25} width={w * 0.25} height={h * 0.35} stroke={color} strokeWidth="1.8" fill={lit ? C.windowGlow : 'none'} />
    <rect x={w * 0.6} y={h * 0.25} width={w * 0.25} height={h * 0.35} stroke={color} strokeWidth="1.8" fill={lit ? C.windowGlow : 'none'} />
    <rect x={w * 0.4} y={h * 0.55} width={w * 0.2} height={h * 0.45} stroke={color} strokeWidth="2" fill="none" />
  </g>
);
