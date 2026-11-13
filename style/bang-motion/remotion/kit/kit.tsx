import React from 'react';
import {AbsoluteFill, interpolate, useCurrentFrame, random} from 'remotion';
import {C, FONT} from './tokens';

export const GlowHorizon: React.FC<{
  glowColor?: string;
  horizonY?: number;
  children?: React.ReactNode;
}> = ({glowColor = C.glow, horizonY = 105, children}) => {
  return (
    <AbsoluteFill style={{
      backgroundColor: C.base,
      backgroundImage: `radial-gradient(75% 45% at 50% ${horizonY}%, color-mix(in srgb, ${glowColor} 30%, transparent), transparent 70%), linear-gradient(180deg, #040508 0%, ${C.base} 55%, #0b1122 100%)`,
      overflow: 'hidden'
    }}>
      {children}
    </AbsoluteFill>
  );
};

export const GrainyMeshBlob: React.FC<{
  x: number; y: number; r: number; color?: string; seed?: string;
}> = ({x, y, r, color = C.accent, seed = 'blob'}) => {
  const f = useCurrentFrame();
  const dx = Math.sin(f / 45) * 40;
  const dy = Math.cos(f / 35) * 35;
  return (
    <div style={{
      position: 'absolute',
      left: x + dx - r,
      top: y + dy - r,
      width: r * 2,
      height: r * 2,
      borderRadius: '50%',
      backgroundColor: color,
      filter: 'blur(90px)',
      opacity: 0.38,
      pointerEvents: 'none'
    }} />
  );
};

export const GlassCard: React.FC<{
  x: number; y: number; w: number; h: number;
  rot?: number; children?: React.ReactNode;
}> = ({x, y, w, h, rot = 0, children}) => {
  return (
    <div style={{
      position: 'absolute', left: x, top: y, width: w, height: h,
      backgroundColor: C.surface,
      backdropFilter: 'blur(20px)',
      WebkitBackdropFilter: 'blur(20px)',
      borderRadius: 24,
      border: `1.5px solid ${C.border}`,
      boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.6), inset 0 1px 1px rgba(255, 255, 255, 0.15)',
      transform: `rotate(${rot}deg)`,
      overflow: 'hidden',
      padding: 32,
      boxSizing: 'border-box'
    }}>
      {children}
    </div>
  );
};

export const KineticTitle: React.FC<{
  text: string;
  highlight?: string;
  size?: number;
}> = ({text, highlight, size = 68}) => {
  return (
    <h1 style={{
      fontFamily: FONT.headline,
      fontSize: size,
      fontWeight: 700,
      color: C.text,
      lineHeight: 1.15,
      letterSpacing: '-0.02em',
      margin: 0,
      textShadow: '0 4px 20px rgba(0,0,0,0.5)'
    }}>
      {text}{' '}
      {highlight && (
        <span style={{
          color: C.accentLit,
          textShadow: `0 0 35px ${C.accent}`
        }}>
          {highlight}
        </span>
      )}
    </h1>
  );
};

export const StatusPill: React.FC<{text: string}> = ({text}) => (
  <div style={{
    display: 'inline-flex', alignItems: 'center', gap: 8,
    padding: '6px 14px', borderRadius: 9999,
    background: 'rgba(56, 189, 248, 0.12)',
    border: `1px solid ${C.glow}`,
    color: C.glow, fontFamily: FONT.mono, fontSize: 20,
    fontWeight: 600, letterSpacing: '0.04em'
  }}>
    <span style={{width: 8, height: 8, borderRadius: '50%', backgroundColor: C.glow}} />
    {text}
  </div>
);
