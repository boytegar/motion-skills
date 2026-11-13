import React from 'react';
import {C, FONT} from './tokens';

export const CrtScanlines: React.FC = () => (
  <div style={{
    position: 'absolute', inset: 0, pointerEvents: 'none',
    backgroundImage: 'linear-gradient(rgba(18, 16, 16, 0) 50%, rgba(0, 0, 0, 0.4) 50%)',
    backgroundSize: '100% 6px', zIndex: 10
  }} />
);

export const PixelBox: React.FC<{
  x: number; y: number; w: number; h: number; bg?: string; border?: string; children?: React.ReactNode;
}> = ({x, y, w, h, bg = C.darkBlue, border = C.white, children}) => (
  <div style={{
    position: 'absolute', left: x, top: y, width: w, height: h,
    backgroundColor: bg,
    border: `6px solid ${border}`,
    boxShadow: `inset 4px 4px 0 ${C.lightGrey}, 6px 6px 0 #000`,
    imageRendering: 'pixelated'
  }}>
    {children}
  </div>
);
