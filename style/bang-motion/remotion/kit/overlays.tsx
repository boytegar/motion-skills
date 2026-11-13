import React from 'react';
import {AbsoluteFill} from 'remotion';
import {C, FONT} from './tokens';

export const BangBanner: React.FC<{text: string; badge?: string}> = ({text, badge = 'BANG'}) => {
  return (
    <div style={{
      position: 'absolute', left: 70, top: 180, width: 940,
      display: 'flex', alignItems: 'center', justifyContent: 'space-between',
      padding: '16px 28px',
      background: 'rgba(17, 24, 39, 0.65)',
      backdropFilter: 'blur(16px)',
      WebkitBackdropFilter: 'blur(16px)',
      border: `1px solid ${C.border}`,
      borderRadius: 16,
      boxShadow: '0 10px 25px -5px rgba(0, 0, 0, 0.4)'
    }}>
      <span style={{
        fontFamily: FONT.headline, fontSize: 32, fontWeight: 700,
        color: C.text, letterSpacing: '-0.01em'
      }}>
        {text}
      </span>
      <span style={{
        fontFamily: FONT.mono, fontSize: 18, fontWeight: 700,
        color: C.accentLit, background: 'rgba(99, 102, 241, 0.2)',
        padding: '6px 12px', borderRadius: 8, border: `1px solid ${C.accent}`
      }}>
        {badge}
      </span>
    </div>
  );
};

export const GlassCaptions: React.FC<{text: string}> = ({text}) => {
  return (
    <div style={{
      position: 'absolute', left: 90, top: 1420, width: 900,
      textAlign: 'center', padding: '16px 24px',
      background: 'rgba(10, 15, 29, 0.82)',
      backdropFilter: 'blur(20px)',
      WebkitBackdropFilter: 'blur(20px)',
      border: `1px solid ${C.borderLit}`,
      borderRadius: 16,
      boxShadow: '0 20px 40px rgba(0,0,0,0.5)'
    }}>
      <p style={{
        margin: 0, fontFamily: FONT.caption,
        fontSize: 34, fontWeight: 500, color: C.text,
        letterSpacing: '-0.01em'
      }}>
        {text}
      </p>
    </div>
  );
};

export const LivingGrain: React.FC = () => {
  return (
    <AbsoluteFill style={{
      pointerEvents: 'none',
      opacity: 0.15,
      mixBlendMode: 'overlay',
      backgroundImage: 'radial-gradient(circle, #ffffff 1px, transparent 1px)',
      backgroundSize: '4px 4px'
    }} />
  );
};
