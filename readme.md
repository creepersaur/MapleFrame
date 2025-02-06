# MapleFrame

An easy to use ImmediateMode gui library for Rust.

Uses macroquad as a backend. Inspired by Dear Imgui.
(This is the remaster of `AmberWindow` which is broken and discontinued.)

## Supported Platforms

- Windows / PC
- Linux (untested)
- MacOS (untested)

## Features

- Easy to setup
- Getting windows working is easy
- Many pre-made widgets to use

# Examples

### Macroquad App

Just the default macroquad start app.

```rs
use macroquad::prelude::*;

#[macroquad::main("Macroquad")]
async fn main() {
	loop { next_frame().await }
}
```

### Hello Window

Create your first window using `MapleFrame`.

```rs
use macroquad::prelude::*;
use mapleframe::prelude::*;

#[macroquad::main("Macroquad")]
async fn main() {
	let windows = WindowManager::new();

	loop {
		windows.begin("Window", |win| {
			// We'll code here later.
		})

		windows.end_windows();
		next_frame().await
	}
}
```

### Text and Buttons

Adding text to a window and button callbacks.

```rs
use macroquad::prelude::*;
use mapleframe::prelude::*;

#[macroquad::main("Macroquad")]
async fn main() {
	let windows = WindowManager::new();

	loop {
		windows.begin("Window", |win| {
			win.text("Hello world!");

			if win.button("Press me!").clicked {
				println!("You clicked me!");
			}
		})

		windows.end_windows();
		next_frame().await
	}
}
```

# Making it look like Dear Imgui

If you want MapleFrame to look like DearImgui, you can use `WindowStyle::imgui()` to get a DearImgui styled window:

```rs
use macroquad::prelude::*;
use mapleframe::prelude::*;

#[macroquad::main("Macroquad")]
async fn main() {
	let windows = WindowManager::new();
	let style = WindowStyle::imgui(); // Returns a window style

	loop {
		windows.begin("Window", |win| {
			// Only set style once
			if win.once {
				win.set_style(&style);
			}
		})

		windows.end_windows();
		next_frame().await
	}
}
