use libc::*;
use std::os::unix::io::AsRawFd;
use std::{fs::File, ptr};

fn main() {
    // Create a kqueue instance
    let kq = unsafe { kqueue() };
    if kq == -1 {
        panic!("Failed to create kqueue");
    }

    // This is file descrpitor.
    println!("Kqueue - {:?}", kq);

    let file = File::open("example.txt").expect("Failed to open file");
    let fd = file.as_raw_fd();

    // Define an event for monitoring read availability
    let mut event = kevent {
        ident: fd as usize,        // The file descriptor
        filter: EVFILT_READ,       // Monitor for read events
        flags: EV_ADD | EV_ENABLE, // Add event and enable it
        fflags: 0,
        data: 0,
        udata: ptr::null_mut(),
    };

    // Register the event with kqueue
    let res = unsafe { kevent(kq, &mut event, 1, ptr::null_mut(), 0, ptr::null()) };
    if res == -1 {
        panic!("Failed to register event");
    }

    // Wait for events
    let mut events = [kevent {
        ident: 0,
        filter: 0,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: ptr::null_mut(),
    }; 10];

    println!("Waiting for events...");

    let num_events = unsafe {
        kevent(
            kq,
            ptr::null(),
            0,
            events.as_mut_ptr(),
            events.len() as i32,
            ptr::null(),
        )
    };

    if num_events > 0 {
        println!("{} event(s) received!", num_events);
        for i in 0..num_events as usize {
            let event = events[i];
            let filter = event.filter;
            let data = event.data;
            let flags = event.flags;

            println!(
                "Event - {:?}, Filter {}, Data: {}, Flags: {}",
                event, filter, data, flags
            );
        }
    } else {
        println!("No events received.");
    }
}
