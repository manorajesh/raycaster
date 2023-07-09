# raycaster
A simple raycaster built with Rust with the help of the amazing guide, [Raycasting](https://lodev.org/cgtutor/raycasting2.html).

## Installation
```
git clone https://github.com/manorajesh/raycaster.git && cd raycaster
cargo run
```

## Usage
Use the arrow keys to traverse the extremely entertaining room.

#### Important Code
The [`draw`](https://github.com/manorajesh/raycaster/blob/b34d95d8dfeea019172e3a7cab736a628120c95d/src/raycaster.rs#L74-L186) function is responsible for casting the rays and rendering them accordingly using the custom [`verline`](https://github.com/manorajesh/raycaster/blob/b34d95d8dfeea019172e3a7cab736a628120c95d/src/main.rs#L78-L103) function