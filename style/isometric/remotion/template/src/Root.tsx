import React from 'react';
import {Composition} from 'remotion';
import {IsometricVideo} from './IsometricVideo';
import EP from '../episode.json';

export const RemotionRoot: React.FC = () => (
  <Composition id={EP.comp} component={IsometricVideo} durationInFrames={150} fps={30} width={1080} height={1920} />
);
