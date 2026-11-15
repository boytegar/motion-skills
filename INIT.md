# Initialize a Motion Series

Use this file when a series has no `agents.md`. Initialize once before `motion-strategy`, `motion-explainer`, or a first episode. The resulting files are the series source of truth; later agents read them and skip engine/style questions.

## Inputs

Find the **series directory**: the folder that will contain `episodes/`. Ask only for missing inputs, in this order.

1. **Engine** — explain briefly, then ask for one choice:
   - `remotion` — React/TypeScript; best default for component-based motion.
   - `fframes` — Rust; native render pipeline.
   - `kyfram` — Logos `.lgv`; one scene script per file; requires the Kyfram CLI.
2. **Style** — inspect `$MOTION_LIB/style/*/style.json` and present every available style as `<title> — <tagline>`. Ask the user to choose its `name`.
3. **Series defaults** — ask for audience level, primary region/language, voice notes, palette/accent, mascot/cast notes, and CTA keyword convention. Accept `TBD` for any unknown value.

Resolve `$MOTION_LIB` from the environment, then `${XDG_CONFIG_HOME:-~/.config}/motion-explainer/config.env` (`MOTION_LIB=`). If unavailable, ask for the library path. Read the selected `style.json` before writing: the selected engine must occur in its `engines` array. If it does not, show only the compatible engines and ask again.

## Write

Create these files in the series directory. Create `episodes/` if absent. Keep the selected values identical across both files.

### `agents.md`

```md
# Motion series agent context

## Initialized defaults

- Engine: `<engine>`
- Style: `<style-name>`
- Style guide: `$MOTION_LIB/style/<style-name>/docs/style-guide.md`
- Style prompt: `$MOTION_LIB/style/<style-name>/docs/style-prompt.md`
- Engine reference: `$MOTION_LIB/style/<style-name>/<engine reference from style.json>`

## Agent rule

Read this file before planning, scripting, storyboarding, building, or rendering. Use the initialized engine and style without asking again. Ask only when the user explicitly requests a change; then update this file and `design.md` together.
```

### `design.md`

```md
# Series design

## Visual system

- Style: `<style title>` (`<style-name>`)
- Intent: `<style tagline>`
- Canonical guide: `$MOTION_LIB/style/<style-name>/docs/style-guide.md`
- Canvas: `<width>x<height>`, `<fps> fps`
- Engine: `<engine>`

## Series defaults

- Audience: `<audience>`
- Region/language: `<region/language>`
- Voice: `<voice notes>`
- Palette/accent: `<palette/accent>`
- Mascot/cast: `<mascot/cast notes>`
- CTA keywords: `<keyword convention>`

## Usage

Treat the style guide as visual authority. Reuse its kit, typography, palette, motion, caption, and audio rules. `agents.md` is the operational default; update both files whenever style or engine changes.
```

Use literal resolved paths in the files when `$MOTION_LIB` is known. Do not invent visual rules already defined by the selected style guide.

## Completion

Report the selected engine and style plus created paths. Future agents must read `<series>/agents.md` first, then `<series>/design.md`; they use those defaults for `motion-strategy` and every downstream stage.
