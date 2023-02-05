# Computer Science Building Block

This is a building block over game of life.

## Running

This example is quite resource intensive; it's recommended that you only use it with the `--release` flag:

```bash
trunk serve --release
```

## Concepts

- Uses [`gloo_timer`](https://docs.rs/gloo-timers/latest/gloo_timers/) to automatically step the simulation.
- Logs to the console using the [`weblog`](https://crates.io/crates/weblog) crate.
