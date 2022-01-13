#[test]
fn test_new(){
    let _ = lock::Lock::new(5);
}
#[test]
fn test_deref(){
    static mut LOCK: lock::Lock<u8> = lock::Lock::new(5);
    let mut guard  = unsafe { LOCK.lock().unwrap() };
    assert_eq!(5, *guard);
    *guard += 1;
    assert_eq!(6, *guard);
}

#[test]
fn test_get_set(){
    static mut LOCK: lock::Lock<u8> = lock::Lock::new(5);
    let mut guard  = unsafe { LOCK.lock().unwrap() };
    assert_eq!(&5, guard.get());
    *guard.get_mut() += 1;
    assert_eq!(&6, guard.get());
}

#[test]
#[should_panic]
fn test_poison(){
    static mut LOCK: lock::Lock<u8> = lock::Lock::new(5);
    std::thread::spawn(|| {
        let _guard  = unsafe { LOCK.lock().unwrap() };
        panic!()//poison the guard
    }).join().unwrap_err();
    let _guard = unsafe { LOCK.lock().unwrap() };
}

#[test]
fn test_force_unlock(){
    static mut LOCK: lock::Lock<u8> = lock::Lock::new(5);
    std::thread::spawn(|| {
        let _guard  = unsafe { LOCK.lock().unwrap() };
        panic!()//poison the guard
    }).join().unwrap_err();
    unsafe{ LOCK.force_unlock() };
    let _guard = unsafe { LOCK.lock().unwrap() };
}