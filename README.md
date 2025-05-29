
# Third Person Bevy game template (WIP)
[![1](https://github.com/user-attachments/assets/18b3a90e-07a0-417d-ba41-69f8717a1ba1)](https://github.com/user-attachments/assets/778dd6c0-3722-42c3-ad4a-f3424ac588e2)
[![2](https://github.com/user-attachments/assets/5b2ce515-2ea8-4c38-8611-c7b41b019f20)](https://github.com/user-attachments/assets/0bbc555b-53ce-4e2a-bdab-d1219f857b6d)
[![3](https://github.com/user-attachments/assets/a19f42a7-fdb2-4302-83e8-f7d531dba166)](https://github.com/user-attachments/assets/8692a08e-73c6-411a-b060-a971470e623a)
[![4](https://github.com/user-attachments/assets/90bda2e8-4b29-4b03-b8d0-1a65094717e6)](https://github.com/user-attachments/assets/72a324f8-195c-4608-aaf2-b88a1ba03896)

This template is based on the awesome [BevyFlock 2D template][BevyFlock] featuring out of the box builds for:
- Windows
- Linux
- macOS
- Web (Wasm)
This template is a great way to get started if you aim to build new 3D RPG [Bevy] game!
Start with a [basic project](#write-your-game) and [CI / CD](#release-your-game) that can deploy to [itch.io](https://itch.io).
You can [try this template in your browser!](https://olekspickle.itch.io/bevy-third-person)

## Best way to start

Install [cargo-generate] and run:
```bash
cargo generate olekspickle/bevy_new_third_person
# or with bevy_cli
bevy new -t=https://github.com/olekspickle/bevy_new_third_person my_bevy_app
```

## Write your game

The best way to get started is to play around with the code you find in [`src/game/`](./src/game).
This template comes with a basic project structure that you may find useful:

| Path                                               | Description                                                        |
| -------------------------------------------------- | ------------------------------------------------------------------ |
| [`src/main.rs`](./src/main.rs)                     | App entrypoint(not much going on there)                            |
| [`src/loading/`](./src/loading)                    | A high-level way to load collections of asset handles as resources |
| [`src/game/`](./src/game)                          | Game mechanics & content(inputs, scene, player control & animation)|
| [`src/audio.rs`](./src/audio.rs)                   | Marker components for sound effects and music                      |
| [`src/dev_tools.rs`](./src/dev_tools.rs)           | Dev tools for dev builds (press \` aka backtick to toggle)         |
| [`src/screens/`](./src/screens)                    | Splash screen, title screen, gameplay screen, etc.                 |
| [`src/ui/`](./src/ui)                              | Reusable UI widgets & theming                                      |

Feel free to move things around however you want, though.

## Features:
- [x] import and usage of game mechanics and parameters from .ron (config, credits) (kudos to Caudiciform)
- [x] simple asset loading from [BevyFlock] example with loading from path addition
- [x] third person camera with [bevy_third_person_camera]
- [x] simple keyboard & gamepad mapping to game actions using [leafwing-input-manager]
- [x] simple scene with colliders and rigid bodies using [avian3d]
- [x] simple player movement using [bevy_tnua]
- [x] simple skybox sun cycle using [bevy atmosphere example], with daynight and nimbus modes
- [x] rig and animations using [Universal Animation Library] from quaternius
- [x] experimental sound with [bevy_seedling] based on Firewheel audio engine (which will probably replace bevy_audio)
- [x] consistent Esc back navigation in gameplay and menu via stacked modals

### TODOs (prioritized)
- [ ] Crouch walk
- [ ] Jump with timer(tricky with tnua jump in air)
- [ ] fix music not starting on re-enter gameplay
- [ ] fix sound difference on spawn and unmute
- [ ] better scene: I'm thinking woods, campfire, a target practice dummies and a big rock to climb
- [ ] fullscreen in web to avoid key clash(like Alt for browser)
- [ ] sky background instead of just void lol
- [ ] Movement sfx: jump, dash, sprint
- [ ] implement different music states(idle, battle, movement)
- [ ] spatial audio demo: boombox emitting background music
- [ ] serialize and save settings
- [ ] keys rebind in settings
- [ ] vault mechanics
- [ ] climb mechanics
- [ ] small door/portal demo
- [ ] do not rotate player on aim(silly bug, check it out - release aim looking to the floor)

## Run your game

We recommend using the [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli) to run your game.
Running your game locally is very simple:

- Use `bevy run` to run a native dev build.
- Use `bevy run web` to run a web dev build.

<details>
    <summary><ins>Running release builds</ins></summary>

    - Use `bevy run --release` to run a native release build.
- Use `bevy run --release web` to run a web release build.
</details>

<details>
    <summary><ins>Installing Linux dependencies</ins></summary>

  If you're using Linux, make sure you've installed Bevy's [Linux dependencies].
  Note that this template enables Wayland support, which requires additional dependencies as detailed in the link above.
  Wayland is activated by using the `bevy/wayland` feature in the [`Cargo.toml`](./Cargo.toml).
</details>

<details>
    <summary><ins>(Optional) Improving compile times</ins></summary>

[`.cargo/config.toml`](./.cargo/config.toml) contains documentation on how to set up your environment to improve compile times.
</details>

WARNING: if you work in a private repository, please be aware that macOS and Windows runners cost more build minutes.
**For public repositories the workflow runners are free!**

## Release your game

This template uses [GitHub workflows] to run tests and build releases.
Check the [release-flow](.github/workflows/release.yaml)

## Known issues

There are some known issues in Bevy that can require arcane workarounds.

### My audio is stuttering on web

Audio in web-builds can have issues in some browsers.
This seems to be a general performance issue with wasm builds in all engines, the best solution is just to artificially extend loading phase(seems to be a solution people go for in other engines)

- If you're using materials, you should force your render pipelines to [load at the start of the game]
- Optimize your game as much as you can to keep its FPS high.
- Apply the suggestions from the blog post [Workaround for the Choppy Music in Bevy Web Builds].
- Advise your users to try a Chromium-based browser if there are still issues.

### My game window flashes white for a split second when I start the game on Windows

The game window is created before the GPU is ready to render everything.
This means that it'll start with a white screen for a few frames.
The workaround is to [spawn the Window hidden] and only [make it visible a few frames later]

### My character or camera movement is choppy

Choppy character movement is often caused by movement updates being tied to the frame rate.
See the [`physics_in_fixed_timestep`] example for how to fix this.

Choppy camera movement is almost always caused by the camera being tied too tightly to a moving target position.
You can use [`smooth_nudge`] to make your camera smoothly approach its target position instead.

## License

The source code in this repository is licensed under any of the following at your option:
- [CC0-1.0 License](./LICENSE-CC0)
- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

## Credits

The [assets](./assets) in this repository are all 3rd-party. See the see [credits](assets/credits.json) for more information.

[avian3d]: https://github.com/Jondolf/avian/tree/main/crates/avian3d
[bevy]: https://bevyengine.org/
[bevy atmosphere example]: https://bevyengine.org/examples/3d-rendering/atmosphere/
[bevy-discord]: https://discord.gg/bevy
[bevy-learn]: https://bevyengine.org/learn/
[bevy_third_person_camera]: https://github.com/The-DevBlog/bevy_third_person_camera
[bevy_tnua]: https://github.com/idanarye/bevy-tnua
[bevy_seedling]: https://github.com/CorvusPrudens/bevy_seedling
[Bevy Cheat Book]: https://bevy-cheatbook.github.io/introduction.html
[BevyFlock]: https://github.com/TheBevyFlock/bevy_new_2d
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[leafwing-input-manager]: https://github.com/Leafwing-Studios/leafwing-input-manager
[Linux dependencies]: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md
[trunk]: https://trunkrs.dev/
[Universal Animation Library]: https://quaternius.itch.io/universal-animation-library
[GitHub workflows]: https://docs.github.com/en/actions/using-workflows

[Workaround for the Choppy Music in Bevy Web Builds]: https://necrashter.github.io/bevy-choppy-music-workaround
[spawn the Window hidden]: https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/window_settings.rs#L29-L32
[make it visible a few frames later]: https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/window_settings.rs#L56-L64
[`physics_in_fixed_timestep`]: https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs
[`smooth_nudge`]: https://github.com/bevyengine/bevy/blob/main/examples/movement/smooth_follow.rs#L127-L142
[load at the start of the game]: https://github.com/rparrett/bevy_pipelines_ready/blob/main/src/lib.rs
