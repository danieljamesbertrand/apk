//! JNI bindings for Android. Built only with the "android" feature.

use std::net::SocketAddr;

use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

/// Connect to the ember server and return the echo response.
/// Called from Kotlin: EmberClient.connect(serverAddr: String): String
#[no_mangle]
pub extern "system" fn Java_com_ember_android_EmberClient_connect(
    env: JNIEnv,
    _class: JClass,
    server_addr: JString,
) -> jstring {
    let addr_str: String = match env.get_string(server_addr) {
        Ok(s) => s.into(),
        Err(e) => return env.new_string(format!("Error: {}", e)).unwrap().into_raw(),
    };

    let server_addr: SocketAddr = match addr_str.parse() {
        Ok(addr) => addr,
        Err(e) => {
            let msg = env.new_string(format!("Invalid address: {}", e)).unwrap();
            return msg.into_raw();
        }
    };

    match ember_native::connect_echo(server_addr) {
        Ok(response) => env.new_string(response).unwrap().into_raw(),
        Err(e) => env.new_string(format!("Error: {}", e)).unwrap().into_raw(),
    }
}
