# Changelog

All notable changes to this project will be documented in this file.

## [0.7.1] - 2024-12-15

### 🚀 Features

- *(channels)* New channel for directories and associated transitions (#130)

### 📚 Documentation

- *(contributing)* Update contributing.md with hot topics and link todo (#129)

### ⚙️ Miscellaneous Tasks

- *(changelog)* Update changelog (auto) (#128)

## [0.7.0] - 2024-12-15

### 🚀 Features

- *(themes)* Add support for ui themes (#114)
- *(cable)* Support cable channel invocation through the cli (#116)
- *(themes)* Add support for global themes background colors (#120)
- *(themes)* More builtin UI themes (#125)

### 🐛 Bug Fixes

- *(previewers)* Handle crlf sequences when parsing ansi into ratatui objects (#119)
- *(stdin)* Trim entry newlines when streaming from stdin (#121)
- *(config)* Better handling of default values (#123)

### 🚜 Refactor

- *(screen)* Extract UI related code to separate crate (#106)
- *(help)* Enable help bar by default and add help keybinding (#122)
- *(config)* [**breaking**] Use `$HOME/.config/television` by default for macOS (#124)

### 📚 Documentation

- *(readme)* Add theme previews and udpate readme structure (#126)

### ⚡ Performance

- Add bench for build results list (#107)
- Merge contiguous name match ranges (#108)
- *(ui)* Improve merging of continuous name match ranges (#109)
- Optimize entry ranges (#110)

### ⚙️ Miscellaneous Tasks

- *(changelog)* Update changelog (auto) (#105)
- *(version)* Bump workspace to 0.7.0 (#127)

## [0.6.2] - 2024-12-06

### 🐛 Bug Fixes

- *(windows)* Use cmd on windows instead of sh (#102)

### ⚙️ Miscellaneous Tasks

- *(changelog)* Update changelog (auto) (#98)
- Use named constant for colors (#99)
- Bump workspace to 0.6.2 (#104)

## [0.6.1] - 2024-12-05

### 🚀 Features

- *(remote)* Distinguish cable channels with a separate icon (#94)

### 🐛 Bug Fixes

- *(cable)* Add cable to unit channel variants (#96)

### 🚜 Refactor

- *(helpbar)* Hide the top help panel by default (#97)

### 📚 Documentation

- *(readme)* Update readme with latest version and fix section link (#93)

### ⚙️ Miscellaneous Tasks

- *(changelog)* Update changelog (auto) (#92)

## [0.6.0] - 2024-12-04

### 🚀 Features

- *(layout)* Allow reversing the layout and placing input bar on top (#76)
- *(cable)* Add support for custom channels (#75)

### 🐛 Bug Fixes

- *(output)* Quote output string when it contains spaces and points to an existing path (#77)
- *(stdin)* Better handling of long running stdin streams (#81)
- *(preview)* Remove redundant tokio task when generating builtin file previews (#86)

### 🚜 Refactor

- *(exit)* Use std::process::exit explicitly (#84)

### 📚 Documentation

- *(install)* Update the installation section of the README (#79)
- *(installation)* Update homebrew installation command (#87)

### ⚡ Performance

- Remove unnecessary clone() calls (#83)
- Skip ratatui span when match at end of string (#91)
- Add cache for icon colors (#89)

### ⚙️ Miscellaneous Tasks

- *(changelog)* Update changelog (auto) (#74)
- *(changelog)* Update changelog (auto) (#85)

## [0.5.3] - 2024-11-24

### 🚀 Features

- *(navigation)* Add action to scroll results list by a page (#72)

### 🐛 Bug Fixes

- Quote file names that contain spaces when printing them to stdout (#51)
- *(entry)* Always preserve raw input + match ranges conversions (#62)

### 🚜 Refactor

- *(picker)* Refactor picker logic and add tests to picker, cli, and events (#57)

### 📚 Documentation

- Terminal emulators compatibility and good first issues (#56)
- *(contributing)* Added TOC and Code of Conduct link (#65)

### ⚡ Performance

- *(preview)* Cap the number of concurrent preview tokio tasks in the background (#67)

### 🎨 Styling

- *(git)* Enforce conventional commits on git push with a hook (#61)

### ⚙️ Miscellaneous Tasks

- Add readme version update to github actions (#55)
- *(update_readme)* Fix `update_readme` workflow (#63)
- *(changelog)* Update changelog action trigger (#68)
- *(changelog)* Update changelog (auto) (#70)
- *(changelog)* Update changelog (auto) (#73)
- Bump crate to 0.5.3 and workspace crates to 0.0.7

### Build

- *(infer)* Drop infer dependency and refactor code to a simpler heuristic (#58)

## [0.5.1] - 2024-11-20

### 📚 Documentation

- Add instructions for installing on Arch Linux (#43)
- *(brew)* Add brew installation method for MacOS to README (#45)
- *(config)* Update docs to mention XDG_CONFIG_HOME precedence on all platform (#48)

### ⚙️ Miscellaneous Tasks

- Add CHANGELOG.md (#44)
- *(config)* Default configuration now uses 100% of terminal screen space (#47)
- *(changelog)* Udpate changelog and add corresponding makefile command (#53)
- *(actions)* Remove changelog update from the main branch
- Bump version to 0.5.1

## [0.5.0] - 2024-11-18

### 🚀 Features

- *(cli)* Allow passing passthrough keybindings via stdout for the parent process to deal with (#39)
- *(ui)* Make the top UI help bar toggleable (#41)

### 🚜 Refactor

- *(config)* [**breaking**] Make action names snake case in keybinding configuration (#40)

### 📚 Documentation

- Update README television version
- Update README television version specifier
- Update README television version

### ⚙️ Miscellaneous Tasks

- *(rustfmt)* Update rustfmt.toml (#42)

## [0.4.23] - 2024-11-16

### 🚀 Features

- *(ui)* Make help bar display optional (#35)

### 🚜 Refactor

- *(configuration)* Modularize code and better handling of default options (#32)

### 📚 Documentation

- *(debian)* Add installation docs for debian-based systems (#33)
- *(config)* Update docs default configuration (#34)

## [0.4.22] - 2024-11-16

### 🐛 Bug Fixes

- *(ghactions)* Only trigger cd workflow on new tags (#22)
- *(config)* Swap out default keymaps with user defined ones instead of stacking (#26)

### 🚜 Refactor

- *(channels)* Converting between entries and channels is now generic over channels (#25)

### ⚙️ Miscellaneous Tasks

- *(deb)* Release deb package for television (#31)
- Update CD workflow
- *(cd)* Fix cd configuration for deb packages
- *(cd)* Fix cd configuration for deb packages
- *(versions)* Bump workspace crates versions

## [0.4.21] - 2024-11-13

### 🐛 Bug Fixes

- *(windows)* #20 respect `TELEVISION_CONFIG` env var on windows (#21)

### ⚙️ Miscellaneous Tasks

- *(nix)* Nix flake shell + rust-toolchain.toml setup (#14)

## [0.4.20] - 2024-11-11

### 🐛 Bug Fixes

- *(windows)* Ignore `KeyEventKind::Release` events (#3)
- *(windows)* Bump television_utils to v0.0.1 (#4)
- *(config)* More consistent configuration file location for linux and macos (#9)
- *(workspace)* Fix cargo workspace dependencies
- *(cargo workspace)* Fix cargo workspace structure and dependencies (#15)

### 🚜 Refactor

- *(workspace)* Reorganize cargo workspace (#12)

### 📚 Documentation

- Add terminal emulators compatibility status
- Fix table alignments
- *(readme)* Update terminal emulators compatibility list (#6)

### ⚡ Performance

- *(preview)* Remove temporary plaintext previews in favor of loading message preview (#10)

### ⚙️ Miscellaneous Tasks

- Update README.md install section
- *(coc)* Create CODE_OF_CONDUCT.md (#7)
- *(crate)* Add include directives to Cargo.toml to make the crate leaner (#11)

## [0.4.18] - 2024-11-10

### 🐛 Bug Fixes

- Add the correct permissions to release binaries
- Add `winapi-util` dependency for windows builds

## [0.4.17] - 2024-11-10

### ⚙️ Miscellaneous Tasks

- Udate documentation and dependencies
- Update Makefile and CONTRIBUTING.md
- Testing out the CD pipeline

## [0.4.15] - 2024-11-10

### 🚀 Features

- Send to channel
- More syntaxes and themes for highlighting + configuration

### 🐛 Bug Fixes

- Fixing various issues
- Fixing various issues
- Fix linting issues and ignore derive docs for tests
- Filtering system directories in gitrepos
- Stabilize preview scroll initialization
- Doctests imports
- Gag stdout and stderr while loading theme assets to silence bat warning

### 🚜 Refactor

- Refactoring
- Refactoring matcher
- Extract matcher logic into separate crate
- Split project into separate crates
- More refactoring and fixing doctests

### 📚 Documentation

- Docs and linting
- Documentation
- Update README.md
- Add default keybindings to README.md
- Some work on CONTRIBUTING.md
- More work on CONTRIBUTING.md

### 🧪 Testing

- Tests and docs for strings.rs
- Testing ci

### ⚙️ Miscellaneous Tasks

- Bump version
- Bump version
- Unused imports and ci docs
- Update dependencies and bump version
- Update dependencies and bump version
- Bump version
- Update dependencies and bump version
- Makefile and dist scripts
- *(precommit)* Don't allow committing if clippy doesn't pass
- Patch
- Update workspace crates configurations
- *(previewers)* Unused attributes
- Add license to syntax snippet
- Bump version

<!-- generated by git-cliff -->
