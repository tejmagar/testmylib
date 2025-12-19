use std::slice;

use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JString};
use jni::sys::{jbyteArray, jint, jsize};

pub mod converter;

pub fn throw_exception<'a>(env: &mut JNIEnv<'a>, message: &str) {
    let _ = env.throw_new("java/lang/Exception", message);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_tej_imageconverter_ImageConverter_bytes_from_raw<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    width: jint,
    height: jint,
    channels: jint,
    row_stride: jint,
    buffer: JByteBuffer<'a>,
    convert_to: JString<'a>,
) -> jbyteArray {
    let ptr = match env.get_direct_buffer_address(&buffer) {
        Ok(ptr) => ptr,
        Err(error) => {
            throw_exception(&mut env, &error.to_string());
            return std::ptr::null_mut();
        }
    };

    let capacity_call_result = match env.call_method(buffer, "capacity", "()I", &[]) {
        Ok(size) => size.i(),
        Err(error) => {
            throw_exception(&mut env, &error.to_string());
            return std::ptr::null_mut();
        }
    };

    let capacity = match capacity_call_result {
        Ok(size) => size as usize,
        Err(error) => {
            throw_exception(&mut env, &error.to_string());
            return std::ptr::null_mut();
        }
    };

    let slice: &mut [u8] = unsafe { slice::from_raw_parts_mut(ptr, capacity) };

    let convert_to: String = env
        .get_string(&convert_to)
        .expect("Failed to get string.")
        .into();
    let result = match converter::bytes_from_raw(
        width as usize,
        height as usize,
        channels as usize,
        row_stride as usize,
        slice,
        convert_to,
    ) {
        Ok(data) => data,
        Err(error) => {
            throw_exception(&mut env, &error);
            return std::ptr::null_mut();
        }
    };

    let result_i8: &[i8] =
        unsafe { std::slice::from_raw_parts(result.as_ptr() as *const i8, result.len()) };

    let array = match env.new_byte_array(result.len() as jsize) {
        Ok(array) => array,
        Err(error) => {
            throw_exception(&mut env, &error.to_string());
            return std::ptr::null_mut();
        }
    };

    if let Err(error) = env.set_byte_array_region(&array, 0, result_i8) {
        throw_exception(&mut env, &error.to_string());
        return std::ptr::null_mut();
    };

    **array
}
