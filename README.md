<div align="center">
  <img height="160" src="/image/fltk.png" />
  <h1 id="title">fltk-rs demos</h1>

  [![Documentation](https://docs.rs/fltk/badge.svg)](https://docs.rs/fltk)
  [![Crates.io](https://img.shields.io/crates/v/fltk.svg)](https://crates.io/crates/fltk)
  [![License](https://img.shields.io/crates/l/fltk.svg)](https://github.com/fltk-rs/fltk-rs/blob/master/LICENSE)
  [![Build](https://github.com/fltk-rs/fltk-rs/workflows/Build/badge.svg?branch=master)](https://github.com/fltk-rs/fltk-rs/actions)
</div>

---
<div style="text-align: justify;">

This repository contains demos and examples of applications built with fltk-rs, a user interface library in Rust based on the Fast Light Toolkit (FLTK).

These are usually associated with video tutorials from this  on youtube. Also the fltk-rs repo has an [examples directory](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples) for several standalone examples. These demo apps mainly show interop with other projects in the Rust ecosystem.


### Content

The demos and examples in this demo show the ability of fltk-rs to interact with other projects in the Rust ecosystem. The following resources are provided:

- Video Tutorials – To make it easy to learn fltk-rs, a tutorial [playlist](https://www.youtube.com/playlist?list=PLHqrrowPLkDu9U-uk60sGM-YWLOJFfLoE) has been created on YouTube that shows how to use the library and develop applications with it.

- Samples directory – Within the fltk-rs repository is a [examples directory](https://github.com/fltk-rs/fltk-rs/tree/master/fltk/examples) that includes stand-alone samples of how to use different components of the library. Examples range from simple windows to more advanced applications with multiple windows and complex controls.

- Demo apps – In addition to the code examples, this demo provides a number of demo apps that demonstrate fltk-rs' ability to integrate with other projects in the Rust ecosystem. These apps include examples of integration with popular projects like Reqwest, Serde, Tokyo, and many others.

### Requirements

To use the demos and examples in this repository, you need to have Rust and the Cargo package manager installed. If you don't have them installed, you can follow the
[official instructions ](https://www.rust-lang.org/tools/install)to install Rust and Cargo on your system.

### Install

To install and run the demos and examples in this repository, follow these steps:

1. Clone this repository to your local machine:

```bash
git clone https://github.com/fltk-rs/demos.git
```

2. Navigate to the cloned directory:

```bash
cd demos
```

3. Compile and run the desired demo:

```bash
cargo run --release
```

### Contribute

If you would like to contribute more examples or improvements to this demo, please create a new fork of the repository and submit your changes via a pull request. We greatly appreciate your contribution and support of the Fltk and the Rust community. It is appreciated to add at the end of the examples, yours with a demonstration via gif and the title of the repository as shown below.

### License

This repository is licensed under the MIT license. You can get more information about the license terms in the LICENSE file of this repository.

---

The current demos include:
* [🏜️ cairo:](/cairo) Use cairo for custom drawing inside fltk widgets
* [🗓️  calendar:](#calendar) Uses the chrono crate to create an fltk calendar dialog.
* [🖍️  csv:](#csv) Uses serde and csv to perform custom drawing of data.
* [🎛️ egui:](#egui) Use fltk as a backend for egui
* [🪐 femtovg:](#femtovg) Uses femtovg for 2D drawing in a GlWindow.
* [📽️ ffmpeg:](#ffmpeg) Uses ffmpeg for software video rendering.
* [✒️ framebuffer:](#framebuffer) Using fltk for framebuffer drawing.
* [🎇 glium:](#glium) Use the glium crate for OpenGL drawing.
* [✨  glow:](#glow) Use the glow crate to do OpengGL drawing.
* [🖌️  glut:](#glut) Use the gl crate (An OpenGL function pointer loader) to do OpenGL drawing.
* [📍  glyphmap:](#glyphmap) Maps glyphs (specifically font icons) to their unicode codepoint.
* [🎞️ gst:](#gst) Use libgstreamer to play a video inside an fltk window
* [🏞️ image:](#image) Uses rust-embed and the image crates to load images into fltk.
* [🔦  libmpv:](#libmpv) use libmpv to play a video inside an fltk GlWindow.
* [📺  libvlc:](#libvlc) Creating a media player using fltk and the vlc crate.
* [🎶 musicplayer:](#musicplayer) Creating a music player using custom widgets and the soloud crate.
* [🎨  opengl:](#opengl) Raw OpenGL drawing in an fltk GlWindow.
* [🎞️ pixels:](#pixels) Use the pixels crate to draw a wgpu accelerated framebuffer.
* [🌌 plotters:](#plotters) Use plotters for live plotting (drawing animations) with fltk.
* [🌈  raqote:](#raqote) Use raqote for custom drawing (paint example).
* [🖼️  tinyskia:](#tinyskia) Use tiny-skia for custom drawing.
* [🌚 speedy2d:](#speedy2d) Uses speedy2D crate to do 2D drawings of a circle and an RGB image in a GlWindow.
* [🖥️ systray:](#systray) Use nwg to create an fltk app with systray functionalities on Windows
* [💻 webview:](#webview) Embeds a webview inside an fltk app.
* [🔘 rounded-svg:](#rounded-svg) Use the svg crate along with fltk to create images with rounded borders.
* [📟 terminal:](#terminal) A minimal terminal emulator.
* [🌐  web-todo:](#web-todo) Creating an async web todo app using fltk, reqwest, serde and tokio.
* [🌐  web-todo2:](#web-todo2) Creating an async web todo app using fltk, surf, serde and async-std.
* [🖊️ wgpu:](#wgpu) Use wgpu-rs for gpu accelerated drawing.

---

<div align="center">

<h3 id="cairo">cairo</h3>

![cairo](cairo/assets/ex.jpg)

<h3 id="calendar">calendar</h3>

![calendar](calendar/assets/calendar.gif)

<h3 id="csv">csv</h3>

![csv](csv/assets/csv.gif)

<h3 id="egui-demo">egui-demo</h3>

![egui](egui/assets/egui-demo.gif)

<h3 id="musicplayer">musicplayer</h3>

![musicplayer](musicplayer/musicplayer.gif)

<h3 id="web-todo">web-todo</h3>

![web-todo](web-todo/web-todo.gif)

<h3 id="opengl">opengl</h3>

![opengl](opengl/opengl.gif)

<h3 id="glut">glut</h3>

![glut](glut/glut.gif)

<h3 id="pixels">pixels</h3>

![pixels](pixels/assets/pixels.gif)

<h3 id="plotters">plotters</h3>

![plotters](plotters/plotters.gif)

<h3 id="raqote">raqote</h3>

![raqote](raqote/raqote.gif)

<h3 id="tinyskia">tinyskia</h3>

![tinyskia](tinyskia/tinyskia.gif)

<h3 id="glow">glow</h3>

![glow](glow/glow.gif)

<h3 id="glium">glium</h3>

![glium](glium/glium.gif)

<h3 id="speedy2d">speedy2d</h3>

![speedy2d](speedy2d/speedy2d.gif)

<h3 id="femtovg">femtovg</h3>

![femtovg](femtovg/assets/femtovg.gif)

<h3 id="webview">webview</h3>

![webview](webview/assets/webview.gif)

<h3 id="glyphmap">glyphmap</h3>

![glyphmap](glyphmap/glyphmap.gif)

<h3 id="terminal">terminal</h3>

![terminal](terminal/terminal.gif)

<h3 id="rounded-svg">rounded-svg</h3>

![rounded-svg](rounded-svg/rounded-svg.gif)

<h3 id="systray">systray</h3>

![systray](systray/assets/systray.gif)

<h3 id="wgpu">wgpu</h3>

![wgpu](wgpu/wgpu.gif)


</div>

<h3 id="cairo_shadow_button">cairo shadow button</h3>

![cairo](cairo_shadow_button/assets/ex1.jpg)

</div>
