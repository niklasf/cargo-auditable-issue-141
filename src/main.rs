fn main() {
    let mut source = &b"foo"[..];
    let _ = zstd::decode_all(&mut source);
}
