use jni::objects::{JByteArray, JClass};
use jni::sys::jint;
use jni::JNIEnv;

#[unsafe(no_mangle)]
pub extern "system" fn Java_day_pithos_yuvutils_YUVUtils_getLength(
    env: JNIEnv,
    _class: JClass,
    input: JByteArray,
) -> jint {
    let data = env
        .convert_byte_array(input)
        .unwrap();

    data.len() as jint
}
