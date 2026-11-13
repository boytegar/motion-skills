import React from 'react';
import {C, FONT} from './tokens';

export const IsometricBanner: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 80, top: 220, width: 920,
    border: `2px solid ${C.edge}`, background: 'rgba(255,255,255,0.95)',
    padding: '14px 20px', fontFamily: FONT.banner,
    fontWeight: 700, fontSize: 36, color: C.accent,
    textAlign: 'center', letterSpacing: '0.04em'
  }}>
    {text}
  </div>
);

export const IsometricCaptions: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 100, top: 1450, width: 880,
    background: C.accent, color: '#ffffff',
    fontFamily: FONT.caption, fontSize: 30, textAlign: 'center',
    padding: '12px 18px', borderRadius: 4
  }}>
    {text}
  </div>
);
