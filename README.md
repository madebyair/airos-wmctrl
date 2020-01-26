# Wmctrl Wrapper 
A wrapper for the command line tool wmctrl written in Rust

## Usage

Add `wmctrl` to your dependencies in your `Cargo.toml`:

```toml
[dependencies]
wmctrl = "0.1.3"
```

If you want the latest build use this GitHub repository as your uplink:
```toml
[dependencies]
wmctrl = { git = "https://github.com/Treborium/rust-wmctrl" }
```

## Examples

Please refer to the [documentation]() for detailed information. 

If you want to copy & paste the examples below you _need to use_ the following import statement: 

```Rust
use wmctrl::{Self, Window};
```

Find a window based on the title:

```Rust
let windows = wmctrl::get_windows();
let firefox = wmctrl::utils::find_window_by_title(&windows, "Firefox").unwrap();
println!("{}", firefox);
```

Resize and move a window to the specified coordinates:

``` Rust
let win = wmctrl::get_windows().get(0).unwrap();
// This will move the window to the top left corner and resize it to 960x540
win.transform(wmctrl::Transformation::new(0, 0, 960, 540));
``` 

Close the window gracefully:

```Rust
// We need to move the window out of the vector so there is no reference left
let win: Window = wmctrl::get_windows().remove(0);
win.close();
```

Make the window fullscreen: 

```Rust
let win = wmctrl::get_windows().get(0).unwrap();
// Make the window fullscreen
win.change_state(wmctrl::State::new(wmctrl::Action::Add, wmctrl::Property::Fullscreen));
```