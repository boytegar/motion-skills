import React from 'react';
import {AbsoluteFill} from 'remotion';
import {ParchmentStage, LineArtWalker, LineArtHouse} from '../lib/kit';

export const S01Hook: React.FC = () => (
  <ParchmentStage timeOfDay="day">
    <svg style={{position: 'absolute', inset: 0, width: '100%', height: '100%'}}>
      <LineArtHouse x={350} y={720} w={380} h={280} lit={true} />
      <LineArtWalker x={240} y={860} scale={1.8} />
    </svg>
  </ParchmentStage>
);
