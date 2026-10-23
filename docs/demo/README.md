# Offline Notes codec example

Recorded on 23 October 2026 from the cleanup checkout. The commit containing these notes identifies the reviewed source.

```sh
CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 cargo build --locked -p icloud-cli
cargo run --locked -p icloud-api --example offline_note
```

The example uses the real notes::markdown and notes::proto modules. It parses fictional Markdown, encodes the internal document, decodes it and renders Markdown again. Assertions check that the fixture text survives. It creates no API client, login or session and makes no network request. Cargo may download build dependencies.

`output.txt` is actual stdout. `offline-note.png` is a browser-rendered transcript, not Apple Notes or a live synced note. The example does not qualify all Markdown formatting, attachments, account authentication, sync, data mutations, shell operations or Apple service compatibility.

The host was Linux aarch64 with Rust 1.96.0. The toolchain file selects the moving stable channel rather than an exact compiler version. No real note content, credentials, caches or account data was used.

The locked CLI build passed. `cargo test --locked -p icloud-api --lib notes::` passed 30 tests, with 20 unrelated tests filtered out and none ignored. This covers existing Notes codecs, tables, cache/store and related pure fixture behavior, not the complete workspace or Apple endpoints. `./target/debug/icloud --help` produced cli-help.txt without logging in.
