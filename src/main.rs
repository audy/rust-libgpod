extern crate libc;

use libc::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

#[allow(warnings)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn get_cstr_from_ptr(ptr: *const c_char) -> String {
    let res = unsafe {
        if ptr.is_null() {
            "null".to_string()
        } else {
            CStr::from_ptr(ptr).to_string_lossy().to_string()
        }
    };
    res
}

fn fetch_tracks(ipod_mountpoint: &str) -> Vec<String> {
    // Convert Rust string to C string
    let c_mountpoint = CString::new(ipod_mountpoint).expect("CString::new failed");

    let mut tracks: Vec<String> = Vec::new();
    unsafe {
        // Open the iPod database
        let db = itdb_parse(c_mountpoint.as_ptr(), ptr::null_mut());

        // if db.is_null() {
        //    eprintln!("Failed to open iPod database.");
        //    return;
        // }

        let mut track = (*db).tracks;

        while !track.is_null() {
            let itdb_track = (*track).data as *mut Itdb_Track;

            let title = get_cstr_from_ptr((*itdb_track).title);
            let artist = get_cstr_from_ptr((*itdb_track).artist);

            let track_str = format!("{} - \"{}\"", artist, title);

            track = (*track).next;

            tracks.push(track_str);
        }

        // Free the iPod database
        itdb_free(db);
    }

    tracks
}

fn main() {
    let ipod_mountpoint = "/mnt/f"; // Change this to your iPod's mount point

    let tracks = fetch_tracks(ipod_mountpoint);

    dbg!(tracks);
}
