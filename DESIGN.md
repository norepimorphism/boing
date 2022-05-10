# Design

*libui-ng* (and *libui*) were apparently not designed with Rust in mind. 😉

To guarantee safety to the end user, both type-level and runtime decisions were made to restrict usage of the *libui-ng* API. As *libui-ng* lacks polished documentation at the time of writing, the following explanations will reference the source code directly.

## Initialization

`uiInit` must be called exactly once.

### Reasons

Calling `uinit` twice...
* ...on Windows returns `ERROR_CLASS_ALREADY_EXISTS` from `RegisterClassEx`.
* ...on macOS duplicates the app delegate and autorelease pool.

### Solution

*boing*'s `Ui::new` sets a global boolean when called for the first time, and aborts if the boolean is already set.

```rust
let _ = ui.new()?;

// ERROR: *libui-ng* is already initialized.
let _ = ui.new()?;
```

## Widget Construction

Widget construction requires that `uiInit` has previously been called.

### Reasons

Constructing a widget before `uiInit` is called...
* ...on Windows *TODO*
* ...on macOS *TODO*
* ...on Linux *TODO*

### Solution

To be construted, *boing* widgets require access to a `Ui` object, which can only be obtained after `Ui::run` and, by extension, `uiInit`, are called.

```rust

```

## Main Loop

Although not explicitly stated, *libui-ng* seems to permit calling `uiMain` and `uiQuit` multiple times. It should likewise be possible to invoke the main loop multiple times from *boing*.

### Solution

```rust
let ui: Ui;
ui.run();

// This is OK (but weird!).
ui.run();
```

## Control Destruction

`uiControlDestroy` must be called on a given control exactly once, after which the control must no longer be accessed.

### Reasons

Destroying a control twice...
* ...on Windows *TODO*
* ...on macOS *TODO*
* ...on Linux *TODO*

Accessing a control after it has been destroyed...
* ...on Windows *TODO*
* ...on macOS *TODO*
* ...on Linux *TODO*

```rust

```
