import React from 'react';
import {C, FONT} from './tokens';

export const SketchBanner: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 90, top: 220, width: 900,
    borderBottom: `3px solid ${C.ink}`,
    fontFamily: FONT.banner, fontSize: 50, color: C.ink,
    textAlign: 'center', padding: '12px 16px'
  }}>
    {text}
  </div>
);

export const SketchCaptions: React.FC<{text: string}> = ({text}) => (
  <div style={{
    position: 'absolute', left: 120, top: 1420, width: 840,
    fontFamily: FONT.caption, fontSize: 40, color: C.graphite,
    textAlign: 'center', padding: '10px 14px'
  }}>
    {text}
  </div>
);
