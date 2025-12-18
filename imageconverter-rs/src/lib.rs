use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};
use jni::JNIEnv;


#[unsafe(no_mangle)]
pub extern "system" fn Java_tej_imageconverter_ImageConverter_convert(
    env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jstring {

    let output = env.new_string(format!("user entered {} {}", a, b))
        .unwrap();

    output.into_raw()
}
