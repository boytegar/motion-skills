# Initialize a Motion Series

Use this file when a series has no `agents.md`. Initialize once before `motion-strategy`, `motion-explainer`, or a first episode. The resulting files are the series source of truth; later agents read them and skip the style question.

## Inputs

Find the **series directory**: the folder that will contain `episodes/`. Ask only for missing inputs, in this order.

1. **Style** — inspect `$MOTION_LIB/style/*/style.json` and present every available style as `<title> — <tagline>`. Ask the user to choose its `name`. All 109 packs are plan-ready; none writes scene code.
2. **Series defaults** — ask for audience level, primary region/language, voice notes, palette/accent, mascot/cast notes, and CTA keyword convention. Accept `TBD` for any unknown value.

Resolve `$MOTION_LIB` from the environment, then `${XDG_CONFIG_HOME:-~/.config}/motion-explainer/config.env` (`MOTION_LIB=`). If unavailable, ask for the library path. Read the selected `style.json` before writing.

## Files

Create these files in the series directory. Create `episodes/` if absent. Keep the selected values identical across both files.

`agents.md`:

```md
# Agents — series defaults

- Style: `<style-name>`
- Style guide: `$MOTION_LIB/style/<style-name>/docs/style-guide.md`
- Style prompt: `$MOTION_LIB/style/<style-name>/docs/style-prompt.md`

Read this file before planning, scripting, storyboarding or building. Use the initialized style without asking again. Ask only when the user explicitly requests a change; then update this file and `design.md` together.
```

`design.md`:

```md
# Design — `<series>`

- Style: `<style title>` (`<style-name>`)
- Intent: `<style tagline>`
- Canonical guide: `$MOTION_LIB/style/<style-name>/docs/style-guide.md`
- Canvas: `<width>x<height>`, `<fps>` fps (from `style.json`, else the guide)

## Series

- Audience: `<audience>`
- Region/language: `<region/language>`
- Voice: `<voice notes>`
- Palette/accent: `<palette/accent>`
- Mascot/cast: `<mascot/cast notes>`
- CTA keywords: `<keyword convention>`

Treat the style guide as visual authority. Reuse its typography, palette, motion, caption, and audio rules. `agents.md` is the operational default; update both files whenever the style changes.
```

Use literal resolved paths in the files when `$MOTION_LIB` is known. Do not invent visual rules already defined by the selected style guide.

Alternatively run `$MOTION_LIB/core/init.sh --series <dir>`, which writes both files from the same inputs.

Report the selected style plus created paths. Future agents must read `<series>/agents.md` first, then `<series>/design.md`; they use those defaults for `motion-strategy` and every downstream stage.
