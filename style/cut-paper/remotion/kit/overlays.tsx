import React from 'react';
import {C, FONT} from './tokens';

export const CutPaperBanner: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 90, top: 220, width: 900,
    background: C.yellow, color: C.ink,
    fontFamily: FONT.banner, fontSize: 40, textAlign: 'center',
    padding: '16px 20px', transform: 'rotate(-1.5deg)',
    filter: 'drop-shadow(3px 6px 10px rgba(0,0,0,0.25))'
  }}>
    {text}
  </div>
);

export const CutPaperCaptions: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 120, top: 1420, width: 840,
    background: C.paper, color: C.ink, fontFamily: FONT.caption,
    fontSize: 36, textAlign: 'center', padding: '10px 18px',
    borderRadius: 8, filter: 'drop-shadow(2px 4px 6px rgba(0,0,0,0.2))'
  }}>
    {text}
  </div>
);
