# fltk-rs-demos

This is a repo for fltk-rs demo examples. These are usually associated with video tutorials from this [playlist](https://www.youtube.com/playlist?list=PLHqrrowPLkDu9U-uk60sGM-YWLOJFfLoE) on youtube.

Also the fltk-rs repo has an [examples directory](https://github.com/MoAlyousef/fltk-rs/tree/master/examples) for several standalone examples.

The current demos include:
- Creating an async web todo app using fltk, reqwest, serde and tokio.
- Creating a media player using fltk and the vlc crate.
- Opengl demo for opengl drawing in an fltk GlWindow.

The demos can be run using:
```
cargo run --bin web-todo
cargo run --bin gltriangle
cargo run --bin vlc-fltk
```

Screenshots:
![alt_test](web-todo/ex.jpg)
![alt_test](libvlc/ex.jpg)
![alt_test](opengl/ex.jpg)
