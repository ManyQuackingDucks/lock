#[test]
fn try_lock_test(){
    static LOCK: lock::Lock<u8> = lock::Lock::new(5);
    let _ = LOCK.try_lock().unwrap();
}

#[cfg(feature = "std")]
#[test]
#[should_panic]
fn try_lock_panic_test(){
    static LOCK: lock::Lock<u8> = lock::Lock::new(5);
    let _guard = LOCK.try_lock().unwrap() ;
    let _ = LOCK.try_lock().unwrap(); //Already locked will panic
}

#[cfg(feature = "std")] //Poisoning doesnt work if std is not avaiable
#[test]
#[should_panic]
fn try_lock_poison_test(){
    static LOCK: lock::Lock<u8> = lock::Lock::new(5);
    std::thread::spawn(|| {
        let _guard  = LOCK.try_lock().unwrap();
        panic!()//poison the guard
    }).join().unwrap_err();
    let _guard = LOCK.try_lock().unwrap();
}