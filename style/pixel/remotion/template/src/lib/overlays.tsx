import React from 'react';
import {C, FONT} from './tokens';

export const PixelBanner: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 60, top: 220, width: 960,
    background: C.yellow, color: C.black,
    border: `6px solid ${C.black}`,
    fontFamily: FONT.banner, fontSize: 28, textAlign: 'center',
    padding: '16px 12px', boxShadow: `6px 6px 0 ${C.darkGrey}`
  }}>
    {text}
  </div>
);

export const PixelCaptions: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 80, top: 1420, width: 920,
    background: C.black, color: C.white, border: `4px solid ${C.white}`,
    fontFamily: FONT.caption, fontSize: 24, textAlign: 'center',
    padding: '12px 14px'
  }}>
    {text}
  </div>
);
