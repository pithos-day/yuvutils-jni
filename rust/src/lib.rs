use jni::JNIEnv;
use jni::objects::{JByteArray, JClass};
use jni::sys::{jbyteArray, jint};
use yuv::{
    YuvPlanarImageMut, YuvChromaSubsampling, YuvRange,
    YuvStandardMatrix, YuvConversionMode, rgb_to_yuv420,
};

#[unsafe(no_mangle)]
pub extern "system" fn Java_day_pithos_yuvutils_YuvUtils_rgbToYuv420(
    mut env: JNIEnv,
    _class: JClass,
    rgb_data: JByteArray,
    width: jint,
    height: jint,
) -> jbyteArray {
    let rgb_bytes = match env.convert_byte_array(&rgb_data) {
        Ok(b) => b,
        Err(e) => {
            env.throw_new("java/lang/RuntimeException", e.to_string()).ok();
            return std::ptr::null_mut();
        }
    };

    let mut planar = YuvPlanarImageMut::<u8>::alloc(
        width as u32,
        height as u32,
        YuvChromaSubsampling::Yuv420,
    );

    let rgb_stride = (width as u32) * 3;

    if let Err(e) = rgb_to_yuv420(
        &mut planar,
        &rgb_bytes,
        rgb_stride,
        YuvRange::Limited,
        YuvStandardMatrix::Bt601,
        YuvConversionMode::Balanced,
    ) {
        env.throw_new("java/lang/RuntimeException", e.to_string()).ok();
        return std::ptr::null_mut();
    }

    let image = planar.to_fixed();
    let mut output = Vec::new();

    output.extend_from_slice(image.y_plane);
    output.extend_from_slice(image.u_plane);
    output.extend_from_slice(image.v_plane);

    match env.byte_array_from_slice(&output) {
        Ok(arr) => arr.into_raw(),
        Err(e) => {
            env.throw_new("java/lang/RuntimeException", e.to_string()).ok();
            std::ptr::null_mut()
        }
    }
}
