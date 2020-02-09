# Changelog

## [0.8.1] - 2020-02-09

### Changed

* Bumped `vte` dep to 0.6.

## [0.8.0] - 2019-12-07

### Removed

* Removed the unicode-normalization feature altogether - it turns out that it
  still has a couple edge cases where it causes incorrect behavior, and fixing
  those would be a lot more effort.

### Fixed

* Fix a couple more end-of-line/wrapping bugs, especially around cursor
  positioning.
* Fix applying combining characters to wide characters.
* Ensure cells can't have contents with width zero (to avoid ambiguity). If an
  empty cell gets a combining character applied to it, default that cell to a
  (normal-width) space first.

## [0.7.0] - 2019-11-23

### Added

* New (default-on) cargo feature `unicode-normalization` which can be disabled
  to disable normalizing cell contents to NFC - it's a pretty small edge case,
  and the data tables required to support it are quite large, which affects
  size-sensitive targets like wasm

## [0.6.3] - 2019-11-20

### Fixed

* Fix output of `contents_formatted` and `contents_diff` when the cursor
  position ends at one past the end of a row.
* If the cursor position is one past the end of a row, any char, even a
  combining char, needs to cause the cursor position to wrap.

## [0.6.2] - 2019-11-13

### Fixed

* Fix zero-width characters when the cursor is at the end of a row.

## [0.6.1] - 2019-11-13

### Added

* Add more debug logging for unhandled escape sequences.

### Changed

* Unhandled escape sequence warnings are now at the `debug` log level.

## [0.6.0] - 2019-11-13

### Added

* `Screen::input_mode_formatted` and `Screen::input_mode_diff` give escape
  codes to set the current terminal input modes.
* `Screen::title_formatted` and `Screen::title_diff` give escape codes to set
  the terminal window title.
* `Screen::bells_diff` gives escape codes to trigger any audible or visual
  bells which have been seen since the previous state.

### Changed

* `Screen::contents_diff` no longer includes audible or visual bells (see
  `Screen::bells_diff` instead).

## [0.5.1] - 2019-11-12

### Fixed

* `Screen::set_size` now actually resizes when requested (previously the
  underlying storage was not being resized, leading to panics when writing
  outside of the original screen).

## [0.5.0] - 2019-11-12

### Added

* Scrollback support.
* `Default` impl for `Parser` which creates an 80x24 terminal with no
  scrollback.

### Removed

* `Parser::screen_mut` (and the `pub` `&mut self` methods on `Screen`). The few
  things you can do to change the screen state directly are now exposed as
  methods on `Parser` itself.

### Changed

* `Cell::contents` now returns a `String` instead of a `&str`.
* `Screen::check_audible_bell` and `Screen::check_visual_bell` have been
  replaced with `Screen::audible_bell_count` and `Screen::visual_bell_count`.
  You should keep track of the "since the last method call" state yourself
  instead of having the screen track it for you.

### Fixed

* Lots of performance and output optimizations.
* Clearing a cell now sets all of that cell's attributes to the current
  attribute set, since different terminals render different things for an empty
  cell based on the attributes.
* `Screen::contents_diff` now includes audible and visual bells when
  appropriate.

## [0.4.0] - 2019-11-08

### Removed

* `Screen::fgcolor`, `Screen::bgcolor`, `Screen::bold`, `Screen::italic`,
  `Screen::underline`, `Screen::inverse`, and `Screen::alternate_screen`:
  these are just implementation details that people shouldn't need to care
  about.

### Fixed

* Fixed cursor movement when the cursor position is already outside of an
  active scroll region.

## [0.3.2] - 2019-11-08

### Fixed

* Clearing cells now correctly sets the cell background color.
* Fixed a couple bugs in wide character handling in `contents_formatted` and
  `contents_diff`.
* Fixed RI when the cursor is at the top of the screen (fixes scrolling up in
  `less`, for instance).
* Fixed VPA incorrectly being clamped to the scroll region.
* Stop treating soft hyphen specially (as far as i can tell, no other terminals
  do this, and i'm not sure why i thought it was necessary to begin with).
* `contents_formatted` now also resets attributes at the start, like
  `contents_diff` does.

## [0.3.1] - 2019-11-06

### Fixed

* Make `contents_formatted` explicitly show the cursor when necessary, in case
  the cursor was previously hidden.

## [0.3.0] - 2019-11-06

### Added

* `Screen::rows` which is like `Screen::contents` except that it returns the
  data by row instead of all at once, and also allows you to restrict the
  region returned to a subset of columns.
* `Screen::rows_formatted` which is like `Screen::rows`, but returns escape
  sequences sufficient to draw the requested subset of each row.
* `Screen::contents_diff` and `Screen::rows_diff` which return escape sequences
  sufficient to turn the visible state of one screen (or a subset of the screen
  in the case of `rows_diff`) into another.

### Changed

* The screen is now exposed separately from the parser, and is cloneable.
* `contents_formatted` now returns `Vec<u8>` instead of `String`.
* `contents` and `contents_formatted` now only allow getting the contents of
  the entire screen rather than a subset (but see the entry for `rows` and
  `rows_formatted` above).

### Removed

* `Cell::new`, since there's not really any reason that this is useful for
  someone to do from outside of the crate.

### Fixed

* `contents_formatted` now preserves the state of empty cells instead of
  filling them with spaces.
* We now clear the row wrapping state when the number of columns in the
  terminal is changed.
* `contents_formatted` now ensures that the cursor has the correct hidden state
  and location.
* `contents_formatted` now clears the screen before starting to draw.

## [0.2.0] - 2019-11-04

### Changed

* Reimplemented in pure safe rust, with a much more accurate parser
* A bunch of minor API tweaks, some backwards-incompatible

## [0.1.2] - 2016-06-04

### Fixed

* Fix returning uninit memory in get_string_formatted/get_string_plaintext
* Handle emoji and zero width unicode characters properly
* Fix cursor positioning with regards to scroll regions and wrapping
* Fix parsing of (ignored) character set escapes
* Explicitly suppress status report escapes

## [0.1.1] - 2016-04-28

### Fixed

* Fix builds

## [0.1.0] - 2016-04-28

### Added

* Initial release
