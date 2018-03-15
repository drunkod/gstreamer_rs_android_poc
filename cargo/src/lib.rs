extern crate gstreamer as gst;

use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern fn rust_greeting() -> *mut c_char {
    //CString::new("hello world".to_owned()).unwrap().into_raw()
    CString::new(gst::version_string().to_owned()).unwrap().into_raw()
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

