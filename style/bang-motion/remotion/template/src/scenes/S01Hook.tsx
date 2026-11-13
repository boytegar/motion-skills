import React from 'react';
import {AbsoluteFill} from 'remotion';
import {GlowHorizon, GrainyMeshBlob, GlassCard, KineticTitle, StatusPill} from '../lib/kit';

export const S01Hook: React.FC = () => (
  <GlowHorizon>
    <GrainyMeshBlob x={300} y={600} r={280} color="#6366f1" />
    <GrainyMeshBlob x={800} y={1100} r={240} color="#38bdf8" />
    <AbsoluteFill style={{display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', padding: 80}}>
      <GlassCard x={90} y={620} w={900} h={640}>
        <div style={{marginBottom: 24}}>
          <StatusPill text="ACTIVE PIPELINE" />
        </div>
        <KineticTitle text="Motion graphics built like" highlight="video." size={64} />
        <p style={{color: '#94a3b8', fontSize: 28, marginTop: 24, lineHeight: 1.4}}>
          One continuous world with glowing horizons, kinetic typography, and zero boring slides.
        </p>
      </GlassCard>
    </AbsoluteFill>
  </GlowHorizon>
);
