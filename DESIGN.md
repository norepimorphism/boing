# Design

*libui-ng* (and *libui*) were apparently not designed with Rust in mind. 😉

To guarantee safety to the end user, both type-level and runtime decisions were made to restrict usage of the *libui-ng* API. As *libui-ng* lacks polished documentation at the time of writing, the following explanations will reference the source code directly.

## Initialization

`uiInit` must be called exactly once.

* On Windows, calling `uiInit` multiple times duplicates calls to `RegisterClassEx`, which returns `ERROR_CLASS_ALREADY_EXISTS` if called a second time. The *libui-ng* utility window will also be created multiple times.

To guarantee this, *boing*'s `Ui::run` sets a global boolean when called for the first time, and aborts if the boolean is already set.

```rust
Ui::run(|_| {});

// ERROR: *libui-ng* is already initialized.
Ui::run(|_| {});
```

## Control Construction

`uiControl` construction requires that `uiInit` has previously been called.

* *TODO*

To guarantee this, to be constructed, all *boing* controls require access to a `Ui` object, which can only be obtained after `Ui::run` and, by extension, `uiInit`, are called.

```rust

```

## Control Destruction

`uiControlDestroy` must be called on a given `uiControl` exactly once, after which the control must no longer be accessed.

* *TODO*

```rust

```

```rust

```
