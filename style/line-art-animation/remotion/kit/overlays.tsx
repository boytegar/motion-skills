import React from 'react';
import {C, FONT} from './tokens';

export const LineArtBanner: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 90, top: 180, width: 900,
    textAlign: 'center', padding: '14px 20px',
    borderTop: `1.5px solid ${C.ink}`,
    borderBottom: `1.5px solid ${C.ink}`,
    color: C.ink, fontFamily: FONT.banner,
    fontSize: 36, letterSpacing: '0.12em', textTransform: 'uppercase'
  }}>
    {text}
  </div>
);

export const LineArtCaptions: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 100, top: 1420, width: 880,
    textAlign: 'center', padding: '14px 20px',
    color: C.ink, fontFamily: FONT.caption,
    fontSize: 34, fontStyle: 'italic', letterSpacing: '0.04em'
  }}>
    {text}
  </div>
);
