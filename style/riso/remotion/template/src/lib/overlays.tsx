import React from 'react';
import {C, FONT} from './tokens';

export const RisoBanner: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 80, top: 220, width: 920,
    border: `4px solid ${C.blue}`, background: C.paper,
    color: C.pink, fontFamily: FONT.banner,
    fontSize: 44, textAlign: 'center', padding: '16px 20px',
    mixBlendMode: 'multiply'
  }}>
    {text}
  </div>
);

export const RisoCaptions: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 100, top: 1420, width: 880,
    background: C.yellow, color: C.blue,
    fontFamily: FONT.caption, fontWeight: 700, fontSize: 34,
    textAlign: 'center', padding: '12px 18px', mixBlendMode: 'multiply'
  }}>
    {text}
  </div>
);
