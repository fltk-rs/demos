# Embedded Webview in an FLTK app

Works on Windows (MSVC toolchain) and Macos. Linux doesn't. Adding support is a hassle, Gtk (and supporting libs) and libwebkit2gtk are required, in addition to the fact that webview accepts a GtkWindow instead of an X11 Window handle, and conversion between both isn't straightforward.

![alt_test](ex.jpg)