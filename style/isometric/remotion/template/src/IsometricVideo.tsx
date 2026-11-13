import React from 'react';
import {AbsoluteFill} from 'remotion';
import {IsometricBanner} from './lib/overlays';
import {SCENES} from './scenes';

export const IsometricVideo: React.FC = () => (
  <AbsoluteFill style={{backgroundColor: '#111'}}>
    {SCENES.map(({id, component: Comp, banner} => (
      <AbsoluteFill key={id}>
        <Comp />
        <div style={{position: 'absolute', top: 120, width: '100%'}}>
          <IsometricBanner text={banner} />
        </div>
      </AbsoluteFill>
    ))}
  </AbsoluteFill>
);
