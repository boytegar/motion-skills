# Imported Lemo-Opuscar styles (99)

Docs-only imports from [Lemo-Opuscar by Lemomo (LemoLab)](https://github.com/lemomo-ai/lemo-opuscar). Style docs are CC BY-NC 4.0; credit Lemo-Opuscar by Lemomo (LemoLab) and link the repo. Regenerate with `python3 tools/lemo_import.py`.

These packs sit in `style/<slug>/` like any other style, but each carries `"status": "docs-only"` and an empty `engines` array. That marking keeps them out of the way of the render pipeline: `motion_renderable` (core/env.sh) is false for them, so `init.sh` will not write an agents.md/design.md naming an engine, `new_episode.sh` will not scaffold them, `install.sh --doctor` does not flag their missing PRELUDE, and `install.sh` does not link their SKILL.md as an agent skill.

They carry the look, motion and sound rules: good for strategy, hook, script and storyboard. To render one, port an engine into `style/<slug>/` and list it in `"engines"`, then drop `"status"`. `python3 tools/lemo_import.py --wire <slug>` does the manifest half of that once the engine files exist.

## 3D & Miniature

| style | title | the look |
|---|---|---|
| `brick-toy` | Brick Toy |  |
| `paper-popup` | Paper Pop-up Book |  |
| `tilt-shift` | Tilt-Shift Miniature |  |
| `lowpoly-island` | Low-poly Isometric Island |  |
| `glass-product` | Glass Product Render |  |
| `memphis-diorama` | Memphis Diorama |  |
| `isometric-picture-book` | Isometric People Scene |  |
| `crayon-world` | Crayon World 3D |  |

## Cartoons

| style | title | the look |
|---|---|---|
| `rubber-hose` | 1930s Rubber Hose Cartoon |  |
| `cel-anime-80s` | 80s Cel Anime |  |
| `scifi-toon` | Sci-Fi Sitcom Toon |  |
| `midcentury-toon` | Mid-century Cartoon |  |
| `comic-history` | Comic History Cartoon |  |
| `manga-panels` | Manga Panels |  |

## Data & Maps

| style | title | the look |
|---|---|---|
| `dataviz` | Data Storytelling |  |
| `white-model` | Projection-Mapped Model |  |
| `data-physical` | Data Physicalization |  |
| `transit-map` | Transit Map |  |
| `map-explainer` | 3D Map Explainer |  |
| `pip-simulation` | Blob Simulation |  |

## Edit & Documentary

| style | title | the look |
|---|---|---|
| `caption-news` | Caption News |  |
| `newsreel` | 1920s Newsreel |  |
| `countdown` | Live Countdown Split-screen |  |
| `city-symphony` | City Symphony |  |
| `visual-investigation` | Visual Investigation |  |
| `kino-montage` | Constructivist Montage |  |
| `telestrator` | Sports Telestrator |  |
| `news-package` | Broadcast News Package |  |

## Film & Tape

| style | title | the look |
|---|---|---|
| `ascii-crt` | CRT ASCII Terminal |  |
| `silent-film` | 1920s Silent Film |  |
| `backrooms` | Liminal Found Footage |  |
| `scratch-film` | Scratch Film |  |
| `double-exposure` | Double Exposure |  |

## Games

| style | title | the look |
|---|---|---|
| `pixel-rpg` | 16-bit Pixel RPG |  |
| `hd-2d` | 2.5D Pixel Art |  |
| `microgame` | Microgame Frenzy |  |
| `game-show` | Rhythm Game Show |  |

## Graphic & Type

| style | title | the look |
|---|---|---|
| `swiss-motion` | Swiss Motion Graphics |  |
| `spy-titles` | 60s Spy Title Sequence |  |
| `art-deco` | Art Deco |  |
| `pictogram-motion` | Pictogram Motion |  |
| `bauhaus-shapes` | Bauhaus Shapes |  |
| `flat-space-explainer` | Scale Zoom Explainer |  |
| `text-flash-cuts` | Text Flash Cuts |  |
| `silhouette-rhythm` | Silhouette Rhythm |  |

## Instruments & Evidence

| style | title | the look |
|---|---|---|
| `satellite-briefing` | Satellite Briefing |  |
| `briefing-board` | Engineering Briefing Board |  |
| `lidar-survey` | LiDAR Survey |  |
| `microscope` | Under the Microscope |  |
| `archive-lighttable` | Archive Light Table |  |
| `security-xray` | Security X-ray |  |
| `dark-documentary` | Radar Sweep Documentary |  |
| `thermal-camera` | Thermal Camera |  |

## Light & Lines

| style | title | the look |
|---|---|---|
| `circuit-board` | Circuit Board |  |
| `firefly-pointillism` | Firefly Pointillism |  |
| `flow-field` | Flow-field Wind Lines |  |
| `oscilloscope-math` | Oscilloscope Math |  |
| `neon-tubes` | Neon Tubes |  |

## Line Diagrams

| style | title | the look |
|---|---|---|
| `blueprint` | Blueprint |  |
| `iso-infographic` | Isometric Infographic |  |
| `teardown` | Exploded Teardown |  |
| `hardcore-info` | Line-Art Cutaway |  |
| `precise-mechanism` | Color-Coded Mechanism |  |
| `instruction-manual` | Instruction Manual Art |  |
| `pen-plotter` | Pen Plotter |  |
| `assembly-line-diagram` | Assembly-Line Diagram |  |

## Painting & Illustration

| style | title | the look |
|---|---|---|
| `crayon-book` | Crayon Picture Book |  |
| `watercolor` | Watercolor Brush |  |
| `impasto` | Impasto Oil Painting |  |
| `urban-sketch` | Urban Sketch · Pen & Wash |  |
| `colored-pencil` | Colored-Pencil Science Illustration |  |

## Print & Printmaking

| style | title | the look |
|---|---|---|
| `risograph` | Risograph Print |  |
| `halftone-dossier` | Halftone Dossier |  |
| `woodcut` | Woodcut Print |  |
| `engraving` | Copperplate Engraving |  |
| `silkscreen-poster` | Silkscreen Travel Poster |  |
| `printed-long-take` | Scratchboard Long Take |  |
| `pop-art` | Pop Art |  |
| `vintage-collage` | Vintage Engraving Collage |  |

## Screens & Interfaces

| style | title | the look |
|---|---|---|
| `dark-keynote` | Dark Tech Keynote |  |
| `living-screencast` | Mascot Screencast |  |
| `hologram-hud` | Sci-fi Hologram HUD |  |
| `desktop-flythrough` | Tabletop App Flythrough |  |
| `spreadsheet` | Spreadsheet Film |  |
| `one-bit-dither` | 1-bit Dither |  |

## Sketch & Board

| style | title | the look |
|---|---|---|
| `one-line` | One-line Drawing |  |
| `whiteboard` | Whiteboard Explainer |  |
| `chalkboard` | Chalkboard |  |
| `sketchnote` | Sketchnote |  |
| `stick-explainer` | Stick-figure Explainer |  |
| `ipad-sketch` | Tablet Sketch Explainer |  |
| `marker-paper` | Marker on Paper |  |

## Traditional Crafts

| style | title | the look |
|---|---|---|
| `ink-wash` | Chinese Ink Wash |  |
| `shadow-puppet` | Shadow Puppetry |  |
| `ukiyoe` | Ukiyo-e |  |
| `papercut-red` | Red Paper-cut |  |
| `paper-lantern` | Paper-cut Lightbox |  |
| `stained-glass` | Stained Glass |  |
| `gilded-illumination` | Gilded Illumination |  |
