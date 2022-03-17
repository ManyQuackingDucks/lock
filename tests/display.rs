#[test]
fn test_already_locked_format() {
    static LOCK: lock::Lock<u8> = lock::Lock::new(5);
    let _guard = LOCK.lock().unwrap();
    eprintln!("{}", LOCK.try_lock().unwrap_err());
}

#[cfg(feature = "std")]
#[test]
fn test_poisoned_format() {
    static LOCK: lock::Lock<u8> = lock::Lock::new(5);
    std::thread::spawn(|| {
        let _guard = LOCK.lock().unwrap();
        panic!() //poison the guard
    })
    .join()
    .unwrap_err();
    eprintln!("{}", LOCK.lock().unwrap_err());
}
