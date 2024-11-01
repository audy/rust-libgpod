extern crate libc;

use libc::{c_char, c_void};
use std::env;
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

#[derive(Debug)]
struct Track {
    title: String,
    artist: String,
    album: String,
}

fn fetch_tracks(ipod_mountpoint: &str) -> Vec<Track> {
    // Convert Rust string to C string
    let c_mountpoint = CString::new(ipod_mountpoint).expect("CString::new failed");

    let mut tracks: Vec<Track> = Vec::new();
    unsafe {
        // Open the iPod database
        let db = itdb_parse(c_mountpoint.as_ptr(), ptr::null_mut());

        dbg!((*db));

        // if db.is_null() {
        //    eprintln!("Failed to open iPod database.");
        //    return;
        // }

        let mut track = (*db).tracks;

        while !track.is_null() {
            let itdb_track = (*track).data as *mut Itdb_Track;

            let title = get_cstr_from_ptr((*itdb_track).title);
            let artist = get_cstr_from_ptr((*itdb_track).artist);
            let album = get_cstr_from_ptr((*itdb_track).album);

            let track_struct = Track {
                title,
                artist,
                album,
            };

            track = (*track).next;

            tracks.push(track_struct);
        }

        // Free the iPod database
        itdb_free(db);
    }

    tracks
}

fn get_device(ipod_mountpoint: &str) {
    // Convert Rust string to C string
    let c_mountpoint = CString::new(ipod_mountpoint).expect("CString::new failed");
    unsafe {
        let db = itdb_parse(c_mountpoint.as_ptr(), ptr::null_mut());
        let device = (*db).filename;
        dbg!(&(*device));
        itdb_free(db);
        device
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let ipod_mountpoint = args.get(1).expect("Error: provide path to iPod");

    println!("Loading iPod mounted @ {}", ipod_mountpoint);

    get_device(ipod_mountpoint);

    let tracks = fetch_tracks(ipod_mountpoint);

    dbg!(tracks);
}
