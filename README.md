# Arusti & PyArusti

Rust OLAN parser

Designed to parse `.seq` files containing One Letter Aerobatic Notation (OLAN) from [OpenAero](https://openaero.net).
Generates a set of Figures containing Elements that define the Radii, Lines, Spins, Rolls etc. that the aircraft
follows for the sequence.

## PyArusti

PyArusti is a Python binding for the parser. At present it contains one function: `parse`. When given an OLAN string,
this will generate a list of figures, each consisting of a list of elements. Each element is a dictionary with the form:

```python
{
    'type': 'Line|Radius|Turn|Stall',
    'attitude': 'Normal|Inverted|KnifeEdge',
    'main_angle': 0,
    'aux_angle': 0,
    'roll_type': 'Standard|Flick|InvertedFlick|Spin|InvertedSpin|HesitationHalves|HesitationQuarters|HesitationEighths',
    'matching': 0
}
```
`main_angle` and `aux_angle` can mean different things depending on the element type. The comments below come from
`arusti/src/arusti/types.rs`:

```rust
/// Main angle defines angle between forward direction and ground
/// Auxiliary angle defines rolls
/// Matching specifies a line matching identifier. All lines with the same identifier in a
///  figure should have the same length.
Line,

/// Main angle defines pull (+ve) or push (-ve) angle
/// Auxiliary angle defines rolls
/// Matching specifies a radius matching identifier. All radii with the same identifier in a
///  figure should have the same radius.
/// If matching is negative, the radius cannot be inverted to make the exit attitude match
Radius,

/// Main angle defines turn angle
/// Auxiliary angle defines roll, positive angle is towards inside, negative is outside
Turn,

/// Main angle defines yaw between entry and exit
/// Auxiliary angle defines pitch between entry and exit
Stall,
```

### Usage

`cargo` will generate `libpyarusti.so` within the `target/debug` folder. Rename this to `pyarusti.so` and move it
somewhere where Python can find it to import it.

```python
import pyarusti
pyarusti.parse("/dq v .''s.''irp...'-~ ~----2j- [0,20] -'',24'' 2> c,24.... [0,22] ~+v-- 4> -id2 2> ''1''m2.' [0,20] ~~++++++2j2 f,2- -22a44")
```