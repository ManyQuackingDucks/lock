#[test]
fn try_lock_test(){
    static mut LOCK: lock::Lock<u8> = lock::Lock::new(5);
    let _ = unsafe { LOCK.try_lock().unwrap() };
}

#[test]
#[should_panic]
fn try_lock_panic_test(){
    static mut LOCK: lock::Lock<u8> = lock::Lock::new(5);
    let _guard = unsafe { LOCK.try_lock().unwrap() };
    let _ = unsafe { LOCK.try_lock().unwrap() }; //Already locked will panic
}

#[test]
#[should_panic]
fn try_lock_poison_test(){
    static mut LOCK: lock::Lock<u8> = lock::Lock::new(5);
    std::thread::spawn(|| {
        let _guard  = unsafe { LOCK.try_lock().unwrap() };
        panic!()//poison the guard
    }).join().unwrap_err();
    let _guard = unsafe { LOCK.try_lock().unwrap() };
}