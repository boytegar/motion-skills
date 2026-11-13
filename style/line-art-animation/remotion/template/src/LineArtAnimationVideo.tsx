import React from 'react';
import {AbsoluteFill} from 'remotion';
import {LineArtBanner} from './lib/overlays';
import {SCENES} from './scenes';

export const LineArtAnimationVideo: React.FC = () => (
  <AbsoluteFill style={{backgroundColor: '#fbf8f1'}}>
    {SCENES.map(({id, component: Comp, banner}) => (
      <AbsoluteFill key={id}>
        <Comp />
        <LineArtBanner text={banner} />
      </AbsoluteFill>
    ))}
  </AbsoluteFill>
);
