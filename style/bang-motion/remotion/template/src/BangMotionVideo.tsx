import React from 'react';
import {AbsoluteFill} from 'remotion';
import {BangBanner, LivingGrain} from './lib/overlays';
import {SCENES} from './scenes';

export const BangMotionVideo: React.FC = () => (
  <AbsoluteFill style={{backgroundColor: '#07090e'}}>
    {SCENES.map(({id, component: Comp, banner}) => (
      <AbsoluteFill key={id}>
        <Comp />
        <BangBanner text={banner} badge="BANG" />
      </AbsoluteFill>
    ))}
    <LivingGrain />
  </AbsoluteFill>
);
