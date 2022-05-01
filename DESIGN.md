# Design

*libui-ng* (and *libui*) were apparently not designed with Rust in mind. 😉

To guarantee safety to the end user, both type-level and runtime decisions were made to restrict usage of the *libui-ng* API. As *libui-ng* lacks polished documentation at the time of writing, the following explanations will reference the source code directly.

## Initialization

`uiInit` must only be called once.

* On Windows, calling `uiInit` multiple times duplicates calls to `RegisterClassEx`, which returns `ERROR_CLASS_ALREADY_EXISTS` if called a second time. The *libui-ng* utility window will also be created multiple times.

To guarantee this, *boing*'s `Ui::run` sets a global boolean when called for the first time, and aborts if the boolean is already set.

```rust

```

## Control Construction

`uiControl` construction requires that `uiInit` has previously been called.

* *TODO*

To guarantee this, to be constructed, all *boing* controls require exclusive access to a `Ui` object, which can only be obtained after `Ui::run` and, by extension, `uiInit`, are called.

```rust

```

## Control Destruction

`uiControlDestroy` must only be called on a given `uiControl` once, after which the control may no longer be accessed.

* *TODO*

To guarantee this, *boing*'s `Ui` type tracks controls as they are constructed and automatically destroys them once dropped, which occurs at the end of `Ui::run`.

```rust

```

Furthermore, the `FnOnce` function passed to `Ui::run` disallows mutation of external state, which prevents users from storing controls outside `Ui::run` and using them after `Ui::drop` destroys their internal handles.

```rust

```
