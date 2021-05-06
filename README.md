# Arusti & PyArusti

Rust OLAN parser

Designed to parse `.seq` files containing One Letter Aerobatic Notation (OLAN) from [OpenAero](https://openaero.net).
Generates a set of Figures containing Elements that define the Radii, Lines, Spins, Rolls etc. that the aircraft
follows for the sequence.

## PyArusti

PyArusti is a pyton binding for the parser. At present it contains one function: `parse`. When given an OLAN string,
this will generate a list of figures, each consisting of a list of elements as a dict with the form:

```python
{
    'type': 'line|radius|turn|roll|flick|spin',
    'inverted': False,
    'angle': 0.0,
    'argument': 0.0
}
```
The angle and argument can mean different things depending on the element type. The comments below come from
`arusti/src/arusti/types.rs`:

```rust
/// Angle defines angle between forward direction and ground
Line,

/// Angle defines pull (+ve) or push (-ve) angle. Argument defines radius matching. -ve argument is non-invertible
Radius,

/// Angle defines turn angle. Argument defines roll, +ve inside, -ve outside
Turn,

/// Angle defines total roll angle. Argument defines hesitation divisions
Roll,

/// Angle defines total roll angle
Flick,

/// Angle defines total spin angle
Spin,
```

### Usage

`cargo` will generate `libpyarusti.so` within the `target/debug` folder. Rename this to `pyarusti.so` and move it
somewhere where Python can find it to import it.

```python
import pyarusti
pyarusti.parse("/dq v .''s.''irp...'-~ ~----2j- [0,20] -'',24'' 2> c,24.... [0,22] ~+v-- 4> -id2 2> ''1''m2.' [0,20] ~~++++++2j2 f,2- -22a44")
```