# Particle Effect with Rust, WASM and HTML Canvas

This project demonstrates how to create a particle system using Rust compiled to WebAssembly (WASM) and rendered on an HTML Canvas.

## Features

- Particles are rendered dynamically on an HTML canvas using WebAssembly for performance.
- Handles dynamic canvas resizing while keeping particle counts consistent.
- Customizable particle behavior with random velocity, direction, and color.
- Integration between Rust and JavaScript for seamless WebAssembly and browser API usage.

## Prerequisites

Before building or running the project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

## Building the Project

To compile the Rust code to WebAssembly, run:

```sh
wasm-pack build --target web
```

This will generate the necessary WebAssembly (`.wasm`) file and a JavaScript wrapper in the `pkg/` directory.

## Running the Project Locally

To serve the project in a browser, you can use Python's built-in HTTP server:

```sh
python3 -m http.server 8080
```

After running the command, open your browser and navigate to:

```
http://localhost:8080
```

## How It Works

- **Rust Core**: The particle system is implemented in Rust, leveraging its performance for real-time updates and physics.
- **WebAssembly**: Rust code is compiled into WebAssembly and invoked via JavaScript.
- **HTML Canvas**: Particles are rendered on the canvas using JavaScript interop with the `draw_particle` function.

## Inspiration

This project is inspired by the tutorial: [Creating a Particle System Using Canvas in Browser](https://www.howtosolutions.net/2022/10/rust-wasm-tutorial-creating-particle-system-using-canvas-in-browser/).

## Customization

Feel free to experiment with:
- The number of particles created.
- Particle size, speed, and colors.
- Canvas dimensions and behavior on resize.
