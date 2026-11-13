import React from 'react';
import {C, FONT} from './tokens';

export const MathBanner: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 80, top: 220, width: 920,
    background: 'rgba(18,19,23,0.92)', border: `2px solid ${C.blue}`,
    color: C.formula, fontFamily: FONT.caption,
    fontSize: 36, textAlign: 'center', padding: '14px 20px',
    borderRadius: 6
  }}>
    {text}
  </div>
);

export const MathCaptions: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 100, top: 1420, width: 880,
    background: '#1a1c24', color: C.formula, borderLeft: `4px solid ${C.green}`,
    fontFamily: FONT.caption, fontSize: 32, textAlign: 'center',
    padding: '12px 16px'
  }}>
    {text}
  </div>
);
