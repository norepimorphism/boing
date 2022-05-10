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
// OK.
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

To be construted, *boing* widgets require access to a `Ui` object, which can only be obtained after `Ui::new` and, by extension, `uiInit`, are called.

```rust
// ERROR: This function doesn't exist!
let window = Window::new(/* ... */);

// OK.
let ui = Ui::new()?;
let window = ui.create_window(/* ... */);
```

## Main Loop

Although not explicitly stated, *libui-ng* seems to permit calling `uiMain` and `uiQuit` multiple times; it should likewise be possible to invoke the main loop multiple times from *boing*. Furthermore, widgets may be modified after `uiMain` or `uiQuit` are invoked.

### Solution

```rust
// OK.
let ui: Ui;
let window = ui.create_window(/* ... */);
ui.run();

// OK (but weird!).
ui.run();
window.set_fullscreen(true);
```

## Control Destruction

`uiControlDestroy` must be called on a given control exactly once, after which the control must no longer be accessed.

### Reasons

Destroying a control twice, as well as accessing a control after it has already been destroyed, causes a use-after-free on all platforms.

### Solution

*boing* controls feature a `Drop` implementation that destroys their *libui-ng* representation, ensuring not only that `uiControlDestroy` is called once and only once, but also that the controls may not be accessed after destruction.

```rust
// OK.
let window: Window;
drop(window);

// ERROR: `window` was already destroyed by its `Drop` implementation.
window.set_fullscreen(true);
drop(window);
```
