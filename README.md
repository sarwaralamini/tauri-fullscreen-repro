# macOS fullscreen reproduction for Tauri #16434

https://github.com/tauri-apps/tauri/issues/16434

The reporter reproduced both symptoms on Apple Silicon macOS by serving this
HTML page to the standalone Tauri app. This repository packages that page as
the default entry point. The page uses CSS and has no application JavaScript.

## Run on macOS

With the Tauri prerequisites installed:

```sh
npm ci
npm run tauri dev
```

Stop any other development server on port 1420 first.

1. Confirm the window shows the blue strip labelled “Top edge of the WebView”.
2. Click the green window control to enter native fullscreen. Wait for the transition to finish.
3. Move the pointer to the top of the screen and use the green control to exit fullscreen.
4. Repeat the transition a few times if necessary.

Expected behavior: the WebView fills its content area and the native title
and window controls remain above the page in windowed mode.

Observed behavior: a large dark gap appears above the page when entering
fullscreen. After exiting, the native title and window controls can overlap
the blue strip.

The Rust manifest pins the affected Tauri, Tao, and Wry versions. Run
`npm run tauri info` from this directory to capture your environment.
