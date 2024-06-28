= What is it?

CRCNS Lens is an overlay on the top of CRCNS.org website.

From a static data repository, to a workspace for electrophysiological analysis.

= How to run

Run with `cargo runcc -c` or

```shell
cargo run --features reload
cargo watch -w lib -x "build -p lib"
```

= References

<https://github.com/rksm/hot-lib-reloader-rs/tree/master/examples/hot-egui>
<https://github.com/parasyte/egui-tokio-example>
<https://github.com/LunchTimeCode/egui_async_http_call/>
