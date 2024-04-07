use send_sync_test::MyAPI;
use send_sync_test::MyImpl;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// an applicaiton Error
#[derive(Debug)]
enum Error {
    Generic,
}

#[derive(Clone)]
struct SharedMutInstance {
    instance: Arc<Mutex<MyImpl>>,
}

impl SharedMutInstance {
    // New creates a new *non-shared* instance. We should only call this once
    // per application.
    // should that "call once" rule be enforced? I don't know. Smells of statics.
    pub fn new() -> SharedMutInstance {
        SharedMutInstance {
            instance: Arc::new(Mutex::new(MyImpl::new())),
        }
    }

    // The main point of the wraper is to limit mutex lock() scopes.
    // This should also help improve error handling instead of using `unwrap()` everywhere.
    pub fn do_something(&mut self) -> Result<i32, Error> {
        match &mut self.instance.lock() {
            Ok(instance) => Ok(instance.do_something()),
            Err(e) => Err(Error::Generic),
        }
    }
}

fn main() -> Result<(), Error> {
    // we should strive to limit the lifetimes of platform impls,
    // so this has a known lifetime.
    // Additional instances should be Clone'd and passes around from this.
    // thus when all instances go out of scope, the impl will be cleaned up.
    // In short: move away from static, shared mutable instances.
    let mut shared_mut = SharedMutInstance::new();

    shared_mut.do_something()?;

    // If another thread (or something) wants access to the same shared instance
    // we have to Clone() it.
    // Calling `new()` will result in an entirely new instance and we'll lose the share.
    let mut thread_instance = shared_mut.clone();
    let x = thread::spawn(move || {
        for i in 0..3 {
            thread::sleep(Duration::from_secs(1));

            if thread_instance.do_something().is_err() {
                break;
            }
        }
    });

    // the original imp should still have access to the shared object
    shared_mut.do_something()?;

    x.join().expect("Thread error.");

    shared_mut.do_something()?;

    Ok(())
}

// we shoud see the Impl drop() clean up things when the last instance goes out of scope.
