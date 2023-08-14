extern crate gst;
extern crate log;

use std::{env, process};
use gst::{prelude::*};
use log::{debug};
//add RUSTFLAGS="-lffi" with error dlopen failed: cannot locate symbol "ffi_type_void" referenced by

use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern fn rust_greeting() -> *mut c_char {
    // Get a string containing the passed pipeline launch syntax
    let pipeline_str = "gst launch";
    
    gst::init().unwrap();
    let gst_version_string = gst::version_string();
    CString::new(format!("hello world {}, {}", pipeline_str, gst_version_string).to_owned()).unwrap().into_raw()
    // Let GStreamer create a pipeline from the parsed launch syntax on the cli.
    // In comparison to the launch_glib_main example, this is using the advanced launch syntax
    // parsing API of GStreamer. The function returns a Result, handing us the pipeline if
    // parsing and creating succeeded, and hands us detailed error information if something
    // went wrong. The error is passed as gst::ParseError. In this example, we separately
    // handle the NoSuchElement error, that GStreamer uses to notify us about elements
    // used within the launch syntax, that are not available (not installed).
    // Especially GUIs should probably handle this case, to tell users that they need to
    // install the corresponding gstreamer plugins.

    // let bus = pipeline.bus().unwrap();

    // pipeline
    //     .set_state(gst::State::Playing)
    //     .expect("Unable to set the pipeline to the `Playing` state");

    // for msg in bus.iter_timed(gst::ClockTime::NONE) {
    //     use gst::MessageView;

    //     match msg.view() {
    //         MessageView::Eos(..) => break,
    //         MessageView::Error(err) => {
    //             println!(
    //                 "Error from {:?}: {} ({:?})",
    //                 err.src().map(|s| s.path_string()),
    //                 err.error(),
    //                 err.debug()
    //             );
    //             break;
    //         }
    //         _ => (),
    //     }
    // }

    // pipeline
    //     .set_state(gst::State::Null)
    //     .expect("Unable to set the pipeline to the `Null` state");

    // debug!("Using {} as player", gst::version_string());
    // let gst_version_string = gst::version_string();
    // CString::new(gst::version_string().to_owned()).unwrap().into_raw()
}

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::JClass;
    use self::jni::sys::jstring;

    #[no_mangle]
    pub unsafe extern fn Java_com_mozilla_gstgreetings_RustGstGreetings_greeting(env: JNIEnv, _: JClass) -> jstring {
        let version_string_ptr = CString::from_raw(rust_greeting());
        let output = env.new_string(version_string_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        output.into_inner()
    }
}

