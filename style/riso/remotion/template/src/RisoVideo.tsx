import React from 'react';
import {AbsoluteFill} from 'remotion';
import {RisoBanner} from './lib/overlays';
import {SCENES} from './scenes';

export const RisoVideo: React.FC = () => (
  <AbsoluteFill style={{backgroundColor: '#111'}}>
    {SCENES.map(({id, component: Comp, banner} => (
      <AbsoluteFill key={id}>
        <Comp />
        <div style={{position: 'absolute', top: 120, width: '100%'}}>
          <RisoBanner text={banner} />
        </div>
      </AbsoluteFill>
    ))}
  </AbsoluteFill>
);
