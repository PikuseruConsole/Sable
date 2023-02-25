# Sable
Sable, Sand, Zobel for [Pikuseru Console](https://github.com/PikuseruConsole/pikuseru-console)

Base code is from [Sandspiel](https://github.com/MaxBittker/sandspiel)

<img src="docs/demo.gif" width="320">

## Features

## Species

  * Wall :Indestructible.
  * Sand :Sinks in water.
  * Water :Puts out fire.
  * Stone :Forms arches, turns into sand under pressure.
  * Ice :Freezes Water, slippery!
  * Gas :Highly Flammable!
  * Cloner :Copies the first element it touches.
  * Mite :Eats wood and plant, but loves dust! Slides on ice..
  * Wood :Sturdy, but biodegradable.
  * Plant :Thrives in wet enviroments.
  * Fungus :Spreads over everything.
  * Seed :Grows on sand, plant, and fungus.
  * Fire :Hot!
  * Lava :Flammable and heavy.
  * Acid :Corrodes other elements.
  * Dust :Pretty, but dangerously explosive.
  * Oil :Produces smoke when set on fire.
  * Rocket :Explodes into copies of the first element it touches.
  * Empty :Erases.

## Build

You need to build it like a wasm project and load it as a cartridge with [pikuseru-console](https://github.com/PikuseruConsole/pikuseru-console):

```
cargo build --release --target wasm32-unknown-unknown
```