# Assets and libraries

World 2's `assets/castle-v082.png` and World 3's `assets/royal-v082.png` are original background paintings generated for this project with OpenAI's built-in image-generation tool. See `docs/ASSETS.md` for provenance. They are embedded in the executable alongside the meadow and galaxy paintings.

The meadow backgrounds in `assets/meadow.png` and `assets/meadow-v2.png` were generated for this project with OpenAI's built-in image-generation tool. The current executable embeds the refreshed v2 background. The generation prompts and provenance are recorded in `docs/ASSETS.md`. Chess pieces, interface, particles and sound effects are original procedural artwork/code. The earlier concept sheet is design reference only and is not embedded in the executable.

Macroquad is distributed under MIT or Apache-2.0; miniquad under MIT or Apache-2.0; quad-snd under MIT or Apache-2.0; serde and serde_json under MIT or Apache-2.0. Other transitive dependencies retain their respective licenses. The package includes a dependency inventory, published crate manifests and license texts under `LICENSES/`.

Nunito is Copyright 2014 The Nunito Project Authors and distributed under the SIL Open Font License 1.1. The game embeds a static weight-700 instance of the upstream variable font, generated with FontTools; its license is included. Macroquad's fallback ProggyClean font is MIT licensed and its upstream notice is included with the crate notices.

World 3 uses cozy-chess 0.3.4 and cozy-chess-types 0.2.2, both MIT licensed, for orthodox move generation and position state. Their upstream manifests and license texts are included under LICENSES.
