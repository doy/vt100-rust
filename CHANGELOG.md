# Changelog

## Unreleased

### Fixed

* Clearing cells now correctly sets the cell background color.

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
