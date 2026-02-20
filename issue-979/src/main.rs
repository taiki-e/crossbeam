fn main() {
    let (s, r) = if cfg!(feature = "array") {
        crossbeam_channel::bounded(10)
    } else {
        crossbeam_channel::unbounded()
    };

    s.try_send("Hello, world!").unwrap();
    // s.send("Hello, world!").unwrap();
    // s.send_timeout("Hello, world!", std::time::Duration::from_secs(1))
    //     .unwrap();
    // s.send_deadline("Hello, world!", std::time::Instant::now())
    //     .unwrap();

    assert_eq!(r.recv(), Ok("Hello, world!"));
}
