# Maybe Atomic

Some embedded systems may or may not support atomics. This crate has the "atomic" feature, enabled by default. Its structures will use the core atomic structures (e.g. `AtomicBool`) internally. Otherwise, it will use standard data types internally.

## License

Licensed under MIT or Apache-2.0 at your option.
