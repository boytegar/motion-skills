import React from 'react';
import {AbsoluteFill} from 'remotion';
import {SketchbookBanner} from './lib/overlays';
import {SCENES} from './scenes';

export const SketchbookVideo: React.FC = () => (
  <AbsoluteFill style={{backgroundColor: '#111'}}>
    {SCENES.map(({id, component: Comp, banner} => (
      <AbsoluteFill key={id}>
        <Comp />
        <div style={{position: 'absolute', top: 120, width: '100%'}}>
          <SketchbookBanner text={banner} />
        </div>
      </AbsoluteFill>
    ))}
  </AbsoluteFill>
);
