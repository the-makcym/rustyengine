# Changelog

All notable changes to this project will be documented in this file.


## [0.1.0] - 2023-06-08

### Added
- `Conf` now can be read from `.toml` file
- `Default` impl for `Conf` and therefore `Default` impl for `Game`
- `HypeEllipse` that defined with its center, semiaxes lengths and it's axes themselves
- `AsEvent` trait for events, requires `From<crossterm::event::Event>`
- `AsEventSys` trait for event systems, it's single instance is stored in `Game`
- `EventQueue` event system that is just queue of obtaining events and furthermore it implements `AsEventSys` handling events consequently
- Traits
  - `AsEntity`: for material that can be indexed inside the `Game` instance with `Uuid` and can store properties within `HashMap` 
  - `AsCollided`: for material that can be collided with `Ray`. Coefficient of `Ray` resizing is returned if collision exists else `-1.0`
  - `AsGameObject`: for material that has not-consistent position and direction in the game
  - `AsEntityList`: for lists that stores `dyn AsEntity`
  - `AsCollidedList`: for lists that stores `dyn AsCollided`
  - Ierarchy: `AsGameObject`: `AsCollided`: `AsEntity`, `AsCollidedList`: `AsEntityList`
- `Canvas` updates its content and stores picture as `Vec<String>` respectively to `charmap` given in the `Conf`
- `console` utils for getting console size, moving cursor and clearing console

### Changed
- `Camera` no longer creates a bunch of rays each frame but rotates one its instance
- `Game` is parametrized with `AsEvent`, `AsEventSys`, `AsCollidedList` types


## [0.0.5] - 2023-05-19

### Added
- `InceptedRays` that stands for bunch of rays having common inception point
- `Camera` has method for creating a bunch of rays: `incepted_rays`, their number and direction response to `Canvas` size and `Camera` prop: `Dir` or `LookAt`
- `Intersection` trait that requires possibility of being intersected with `Ray`. It also requires `Entity`
- `HypePlane` that are defined with some point on it and normal vector
- `Conf` struct with settings for `Game`
- `Default` implementations for `VectorSpace`, `Point`, `CoordSys`, `Ray`, `Conf`, `Game`
- `Game` constructor immediately creates also `Camera` and `Canvas` that stored as `Game`'s fields
- Checks for whether `initpt` and `space` in `CoordSys` or `inc` and `dir` in `Ray` have exactly the same dimension
- `dim()` methods in `VectorSpace`, `Point`, `CoordSys`
- `ReRes` extended `NoneOpt` variant that stands for success computation yet without useful value
- `sub()` method in `Point` that subtracts themselves

### Changed
- `Point` constructor now takes one-dim vector as the only parameter
- `EntityList` now stores `dyn Intersection` instead of `dyn Entity`


## [0.0.4] - 2023-05-18

### Added
- `Grid` enum responsible for storing arbitrary type with all the features like access by index, transposing, 
getting iterators. Furthermore, it stores the way how to treat the content (by the way, content is presented in 
struct `RawGrid` that is responsible whether `Vec<T>` or `Vec<Vec<T>>` to choose, transposing flags and so on). Such ways
listed in enum `Repr` - it's `Arbitrary`, `Sqaure`, `Row`, `Col`, `MultiRow`, `MultiCol`, `Failure`. The last one added in purpose
of contigious observation whether the failure happens.
- `Matrix` is just wrapper upon `Grid` with `f64` instead of generic `E`. It can do arithmetic ops on top of existing possibilities.
- `ReErr` is the error enum, that have `EngnErr`, `GridErr`, `MathErr` variants (list will be extended).
- Errors implements `std::error::Error` and requires on creation additional information about error. As benefit, they provide `dbg`
method, that describes error in human language
- `ReRes` is just wrapper upon `Result` with `ReErr` instead of generic error type

### Changed
- All the code have been splitten into `lib` and `bin`. First is intended to contain all reusable code.
- Structs related to matrices have been completely changed. See *added* for more information.
- _Cargo.lock_ now is in _.gitignore_ by the convention of how library dependencies works.

### Removed
- All the old staff related to matrices and `Matrixify` trait

### Dependencies
- `thiserror` used for implementing `std::error::Error` for `ReErr`
- `strum` and `strum_macros` used for implementing `std::fmt::Display` for `enum`s


## [0.0.3] - 2023-05-04

### Added
- `Ray` struct using `Point` and `Vector` in the given `CoordSys`
- `IdSet` uses crate `uuid`. More specifically it uses UUID standard v4. It's prohibited to mutate `Id`. 
- `EntityCore` used in every particular `Entity` (following principle Composition Over Inheritance).
It's intended to create it within `Game` instance
- Properties in `EntityCore` are set via enums `Prop` and `AnyVal`
- `Entity` is the enum with variants of different entities
- `EntityList` instance is used in `Game` for storing all related to the entities within game `Id`s 
- `Game` that responsible for storing current `CoordSys` and `EntityList` and running related scripts
- `GameObject` that stands for basic game object
- `GameCamera` with four different constructors
- Reexports into scope of namespace `engine`

### Changed
- `linalg` module renamed to `linal`
- `enums` module rightfully renamed to `errs`

### Fixed
- `VecSpace` now knows precisely whether the basis is orthogonal or not

### Dependecies
- `Uuid` crate with feauture `v4`


## [0.0.2] - 2023-04-13

### Added
- Constructors for rotational matrices

### Changed
- Global singleton ```Gramm``` matrix renamed to ```Gram``` matrix
- ```Vecspace``` struct renamed to ```VecSpace``` struct

### Fixed
- No more panic in scalar product if operands are transposed incorrectly

### Removed
- Pointless operation of addition, subtraction between ```Matrixify``` implementor and number
- Enum ```MatrixType``` that had no usages


## [0.0.1] - 2023-03-30

### Added

- ```Matrixify``` trait that intended to be implemented for each struct
that may be represented as a table of numbers
- ```Matrix``` struct and Matrixify implementation for it
- ```Vector``` struct and Matrixify implementation for it
- Arithmetic operations such as addition, subtraсtion, multiplication,
division between ```Matrix``` and ```Vector```, between them two and numbers
- Scalar product between two ```Vectors``` in basis of vector space or without it
- Vector product between two ```Vectors```
- ```VecSpace``` struct contains basis
- ```Point``` struct as just another representation of Vector in basis
- ```CoordSys``` struct defined with ```VecSpace``` and initial ```Point```
- Global singletones: matrix of bilinear form, actual coordinate system,
corresponding Gram matrix