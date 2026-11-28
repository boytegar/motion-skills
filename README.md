# motion-explainer

Skills untuk membuat video vertical explainer (~20–45 s) dari sebuah link — repo, halaman produk, atau pengumuman — sampai **plan siap-build**.

Pipeline ini **plan-only**: skill di sini menulis brief, hook, script, storyboard, timing suara, dan kontrak handoff. Skill di sini **tidak** menulis kode scene dan tidak merender. Folder plan diserahkan ke builder pilihanmu lewat `HANDOFF.md`, di luar skill set ini.

- Style: 109 style di `style/` (papercut, sketchbook, blueprint, news-package, …). Lihat [STYLE-GALLERY.md](STYLE-GALLERY.md) untuk contoh visual semua style. Default diambil dari `agents.md` series.
- Canvas default: `1080x1920`, 30 fps.
- Bahasa dokumen & skill: mengikuti `SERIES.md`.

---

## 1. Flow pipeline

```
        ┌─────────────────────────── PLAN ONLY (skill ini) ───────────────────────────┐
        │                                                                             │
  link  ▼                                                                             │
 ┌──────────────┐   ┌────────────┐   ┌────────────┐   ┌───────────────┐               │
 │ 0. motion-   │   │ 1. motion- │   │ 2. motion- │   │ 3. motion-    │               │
 │    setup     │──▶│  strategy  │──▶│  hook      │──▶│  script       │               │
 └──────────────┘   └────────────┘   └────────────┘   └───────────────┘               │
   install/doctor     SERIES.md        HOOK block       facts.md                         │
                      BRIEF            STORYBOARD.md    script.json                      │
                      episode.json                      staging table                    │
                                                            │                            │
                                                            ▼                            │
                                             ┌───────────────────────────┐               │
                                             │ 4. motion-storyboard      │               │
                                             │   Camera/Cut table        │               │
                                             │   scene blocks + SFX      │               │
                                             └───────────────────────────┘               │
                                                            │                            │
                                                            ▼                            │
                                             ┌───────────────────────────┐               │
                                             │ 5. motion-voice           │               │
                                             │   WAV per baris + timing  │               │
                                             │   captions + plan.json    │               │
                                             └───────────────────────────┘               │
                                                            │                            │
                                                            ▼                            │
                                             ┌───────────────────────────┐               │
                                             │ 6. motion-build           │  ← END PLAN   │
                                             │   episode.json final      │               │
                                             │   cues.json + HANDOFF.md  │               │
                                             └───────────────────────────┘               │
        └─────────────────────────────────────────────────────────────────────────────┘
                                    │ HANDOFF.md
                                    ▼
        ┌───────────── BUILDER (di luar skill set ini) ──────────────────┐
        │  engine, tool, atau orang yang build folder ini                │
        │  scene code → mix → render → mp4                              │
        └───────────────────────────────────────────────────────────────┘
                                    │ masih + stills
                                    ▼
                       ┌────────────────────────────┐
                       │ 7. motion-qa               │  builder-side (stills)
                       │   frame vs storyboard      │
                       └────────────────────────────┘
                                    │ video sudah posting
                                    ▼
                       ┌────────────────────────────┐
                       │ 8. motion-strategy (loop)  │
                       │   autopsy → SERIES.md      │
                       └────────────────────────────┘
```

File adalah state. Tiap stage baca file stage sebelumnya dan tulis file miliknya, jadi sesi baru bisa lanjut dari stage mana pun.

---

## 2. Fungsi tiap skill

### Planning pipeline (urutan eksekusi)

| # | Skill | Fungsi | Input | Output | Install |
|---|-------|--------|-------|--------|---------|
| 0 | `motion-setup` | Install / cek / upgrade toolchain. Wrapper `install.sh`, pilih komponen, benahi error dependency. | pilihan user | skill ter-link, `config.env`, venv | — |
| 1 | `motion-strategy` | Owner `SERIES.md` (step 0). Pilih topik + satu format familiar (kontes, sidang, pitch show…), funnel keyword komentar, caption, studi kompetitor, autopsy pasca-posting. Tulis BRIEF + scaffold folder episode. | link/sumber, `SERIES.md` | `SERIES.md`, `research/facts.md` (BRIEF), `video/episode.json` identitas | plan only |
| 2 | `motion-hook` | Tulis 3 lapis hook (SAY / WRITE / SHOW / ALIGN), 5 varian diskor 6 pertanyaan, judul cover frame-0, motion keluar dari hook. | BRIEF | HOOK block di `video/STORYBOARD.md` | plan only |
| 3 | `motion-script` | Riset fakta bersumber, script 75–95 kata di 10-beat spine, track banner (10–18 banner), staging table (set/prop/UI/mascot/gag per banner), humour pass. | BRIEF, HOOK | `narration/script.json`, `facts.md`, staging table | plan only |
| 4 | `motion-storyboard` | Camera / Cut table per scene (bahasa efek, bukan sintaks builder), scene block per frame per kata stressed, SFX plan, proof beat, callback. | `script.json`, staging table | tabel + scene blocks + SFX di `STORYBOARD.md` | plan only |
| 5 | `motion-voice` | Voiceover + timing. 4 jalur: TTS hosted dari `.env` (`TTS_PROVIDER`), Qwen3-TTS lokal, rekaman/ASR sendiri, placeholder. Pace tiap baris diukur dari audionya (`core/tools/pace.py`) supaya caption ikut kecepatan suara. Hasil: 1 WAV per baris, `timing_narrator.js`, `captions.json`, `src/plan.json`. | `script.json`, `voice/voice.json` | `narration/voice/final_narrator/*.wav`, `timing_narrator.js`, `public/vo/captions.json`, `src/plan.json` | plan + audio |
| 6 | `motion-build` | Tutup pipeline. Finalkan `episode.json` (scenes/banners/hook/cuts/bed), cue list SFX, `HANDOFF.md` berisi kontrak untuk builder. **Stop — tidak render.** | `STORYBOARD.md`, `script.json`, timing, WAV | `episode.json`, `plan.json`, `captions.json`, `voiceover.wav`, `tools/cues.json`, `HANDOFF.md` | plan + audio |
| 7 | `motion-qa` | **Builder-side.** Bandingkan still hasil render vs storyboard, frame scene, caption, checklist style DNA, tulis delivery summary. Tidak pernah plan/scaffold/pilih renderer. | stills + `plan.json`, `captions.json`, `cues.json`, `HANDOFF.md` | daftar fix, delivery summary | plan + audio |
| 8 | `motion-strategy` (autopsy) | Masukkan angka platform 2 h/24 h ke `research/analytics/`, diagnosis dari hook → topik → format → audience → timing, update `## Evidence` di `SERIES.md`. | insights platform | `analytics/YYYY-MM-DD-autopsy.md` | plan only |

### Router & style

| Skill | Fungsi |
|-------|--------|
| `motion-explainer` | Entry point. Tentukan run mode (**full reel / script only / storyboard-only / style prompt / fix one scene**), intake brief, lalu serahkan ke stage 1. Kasih style prompt portable untuk tool lain. |
| `style/<nama>/SKILL.md` | Satu skill per style (papercut, sketchbook, blueprint, …). Isi: Style DNA, palet, font, gerak, caption, audio, plus daftar komponen yang bisa diminta plan. |
| `motion-style-extract` | Video acuan → style pack baru di `style/<nama>/`. Ukur klip (`core/tools/style_ref.py ingest`: canvas, cut rate, gerak, palet, zona), baca sheet, tulis `SKILL.md` + `style-guide.md` + `style-prompt.md` + token, lalu gate `lint` / `probe`. |
| [STYLE-GALLERY.md](STYLE-GALLERY.md) | Contoh visual tiap style (`sample.png` / `poster.jpg`), dikelompokkan per kategori. Pilih style dari sini sebelum masuk stage 1. |

---

## 3. Run mode (tanya dulu, jangan asumsi)

| Mode | Stage yang jalan | Berhenti di |
|------|------------------|-------------|
| **Full reel** | 1→8 | mp4 (build dilakukan builder pilihanmu) |
| **Script only** | 1→3 | `script.json` + staging table |
| **Storyboard only** | `motion-storyboard` dari script lama | `STORYBOARD.md` |
| **Style prompt** | bagikan blok dari `style/<s>/docs/style-prompt.md` | — |
| **Style dari video** | `motion-style-extract` ( ingest → baca → scaffold → mold → lint ) | `style/<nama>/` siap dipakai stage lain |
| **Fix/rebuild satu scene** | `motion-build` (re-plan) → `motion-qa` | — |

---

## 4. Install mode

| Mode | Isi | Cukup untuk |
|------|-----|-------------|
| Plan only | skill + style docs | strategy, hook, script, storyboard, style prompt |
| Plan + audio files | + `ffmpeg`, Python (numpy/Pillow/soundfile), placeholder VO | caption track, cue list, timing (suara silent/`say`) |
| Plan + real voiceover | + venv Qwen3-TTS, model 1.7B/0.6B, ASR | voiceover asli + word timing |

```sh
./install.sh --doctor                              # cek apa yang ada
./install.sh --mode custom --components skills,deps,pytools,verify --yes
./install.sh --mode custom --components skills,deps,pytools,tts,verify --yes --tts-model 1.7B
./install.sh --dry-run --mode custom --components skills --yes   # lihat rencana dulu
./install.sh --uninstall                           # hapus link yang dibuat
```

- `--target auto|all|claude,codex,gemini,...` — harness tujuan (tabel: `harnesses.sh`).
- `--project DIR` — install per-project, bukan user-wide.
- `--copy` — snapshot, bukan symlink (symlink ikut `git pull`).
- Konfirmasi sebelum download multi-GB.

---

## 5. Struktur project

```
<series>/
  agents.md                 style default (dibuat sekali, WAJIB dibaca dulu)
  design.md                 style DNA, canvas, palette, audience
  SERIES.md                 audience, bahasa, voice, palette, mascot, keyword log
  voice/voice.json          setting voice (copy dari core/voice/voice.example.json)
  research/analytics/       studi kompetitor + autopsy
  episodes/NN-slug/
    research/facts.md       BRIEF + fakta bersumber
    narration/script.json   [{"id": "l1_hook", "text": "..."}, ...]
    narration/voice/final_narrator/<id>.wav
    narration/timing_narrator.js
    video/
      STORYBOARD.md         HOOK → staging table → Camera/Cut → scene blocks → SFX → Timing
      episode.json          scenes, banners, hook, cuts, bed
      src/plan.json         durasi scene + kata cue (dibuat build_vo.py)
      public/vo/            voiceover.wav + captions.json
      tools/cues.json       semua sound cue + waktu
      HANDOFF.md            kontrak untuk builder
```

Builder pilihanmu menambah artefak build-nya sendiri (`video/scenes/*`, `out/*.mp4`, dst).

---

## 6. Alur cepat (dari link ke plan)

```sh
# 0. sekali per series
./install.sh --doctor
$MOTION_LIB/core/init.sh --series ~/videos/my-series   # tulis agents.md + design.md

# 1. strategy
#    → SERIES.md, BRIEF di research/facts.md, folder episode, episode.json
# 2. hook       → HOOK block di STORYBOARD.md
# 3. script     → narration/script.json + staging table
# 4. storyboard → Camera/Cut table + scene blocks + SFX plan
# 5. voice      → WAV, timing, captions, plan.json
# 6. build      → episode.json final, cues.json, HANDOFF.md   ← STOP
```

Stage 5 (voice) dari folder `episode/narration`:

```sh
cp $MOTION_LIB/core/voice/voice.example.json ../../voice/voice.json
$MOTION_LIB/core/voice/narrate.sh ../../voice/voice.json .
$MOTION_LIB/core/voice/finalize.sh ../../voice/voice.json . voice/takes/narrator   # + pace + timing
python3 $MOTION_LIB/core/tools/placeholder_vo.py --wps 3.5    # jalur cepat tanpa TTS
```

TTS hosted: salin template `core/config.env.example` ke `~/.config/motion-explainer/config.env` (`chmod 600`), lalu isi `TTS_PROVIDER=openai|elevenlabs|custom` (atau cukup `TTS_URL` untuk server sendiri) + `TTS_API_KEY` (opsional `TTS_MODEL`, `TTS_VOICE`, `TTS_SPEED`, `TTS_LANG`, `PACE_TARGET`). `TTS_URL` mengganti endpoint vendor, dan tanpa `TTS_PROVIDER` otomatis jadi provider `custom`. `narrate.sh` membaca file itu dan otomatis memakai `core/voice/tts_provider.py` alih-alih model lokal; tanpa `TTS_PROVIDER`, jalur Qwen3-TTS lokal tetap.

Stage 6 (build) dari `episode/video`:

```sh
python3 $MOTION_LIB/core/tools/build_vo.py    # voiceover.wav, captions.json, plan.json
python3 $MOTION_LIB/core/tools/timing_md.py  # section Timing di STORYBOARD.md
python3 $MOTION_LIB/core/tools/cues.py       # tools/cues.json
```

Lalu serahkan `HANDOFF.md` ke builder yang kamu pakai. Setelah ada still hasil render → `motion-qa`.

---

## 7. Aturan lintas stage

- **`agents.md` menang.** Kalau ada, style tidak ditanyakan lagi.
- **`SERIES.md` hanya ditulis `motion-strategy` step 0.** Stage lain cuma baca; autopsy hanya menambah `## Evidence`.
- **Tidak ada nama builder di plan.** Builder hanya muncul di `HANDOFF.md`. Tidak ada key `"engine"` di `episode.json`.
- **Tidak ada kode scene / render di planning.** Render hanya kalau diminta.
- **Reuse sebelum menulis:** komponen kit, props set, scene template, SFX/bed berdasarkan nama dari `core/audio/CATALOG.md`.
- **Trap & batas:** 22–30 s, 75–95 kata, minimal satu limitasi jujur, stack fitur tepat 3, semua angka harus dari `facts.md`, maksimal satu page-turn/tear per video.
- **Paralel:** grup scene, backdrop, dan prop independen boleh dikerjakan agent sekaligus.

---

## 8. Troubleshooting cepat

| Gejala | Fix |
|--------|-----|
| Skill tidak muncul di agent | Restart harness (opencode reload otomatis), cek folder di `harnesses.sh`, `./install.sh --doctor` |
| `MOTION_LIB` tidak ketemu | isi `MOTION_LIB=` di `~/.config/motion-explainer/config.env` atau `export` di shell |
| Caption melenceng vs audio | ASR belum ada → install komponen `tts`, atau rekam manual jalur B |
| `ffmpeg` hilang | `--components deps,verify` |
| Venv voice: "need Python 3.10–3.12" | install `uv` atau `python@3.12`, ulang komponen `tts` |
| Scene < 1.2 s | merge ke scene tetangga, rerun `build_vo.py` |
| Air SFX terlalu rapat | `cues.py` kasih catatan; target ≤ 1 per 2.5 s |
| Butuh renderer | itu builder/renderer pilihanmu, bukan di sini |

---

## Referensi

- Alur & stage lengkap: `skills/motion-explainer/SKILL.md`
- Library core & tools: `core/README.md`
- Voice: `core/voice/README.md`
- Init series: `INIT.md` · Harness: `harnesses.sh`
- Style: `style/<nama>/docs/style-guide.md` (Style DNA = section 2), `docs/style-prompt.md`
