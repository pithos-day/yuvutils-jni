use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JByteBuffer};
use jni::sys::{jbyteArray, jint};
use yuv::{
    BufferStoreMut, YuvPlanarImageMut, YuvChromaSubsampling, YuvRange,
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

#[unsafe(no_mangle)]
pub extern "system" fn Java_day_pithos_yuvutils_YuvUtils_bgrToYuv420(
    mut env: JNIEnv,
    _class: JClass,
    bgr_data: JByteArray,
    width: jint,
    height: jint,
    stride: jint,
) -> jbyteArray {
    let bgr_bytes = match env.convert_byte_array(&bgr_data) {
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

    if let Err(e) = yuv::bgr_to_yuv420(
        &mut planar,
        &bgr_bytes,
        stride as u32,
        YuvRange::Limited,
        YuvStandardMatrix::Bt601,
        YuvConversionMode::Balanced,
    ) {
        env.throw_new("java/lang/RuntimeException", e.to_string()).ok();
        return std::ptr::null_mut();
    }

    let image = planar.to_fixed();
    let mut output = Vec::with_capacity(image.y_plane.len() + image.u_plane.len() + image.v_plane.len());
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

#[unsafe(no_mangle)]
pub extern "system" fn Java_day_pithos_yuvutils_YuvUtils_bgrToYuv420Direct(
    mut env: JNIEnv,
    _class: JClass,
    bgr_buf: JByteBuffer,
    out_buf: JByteBuffer,
    width: jint,
    height: jint,
    stride: jint,
) {
    let bgr_ptr = match env.get_direct_buffer_address(&bgr_buf) {
        Ok(p) => p,
        Err(e) => { env.throw_new("java/lang/RuntimeException", e.to_string()).ok(); return; }
    };
    let bgr_cap = match env.get_direct_buffer_capacity(&bgr_buf) {
        Ok(c) => c,
        Err(e) => { env.throw_new("java/lang/RuntimeException", e.to_string()).ok(); return; }
    };
    let out_ptr = match env.get_direct_buffer_address(&out_buf) {
        Ok(p) => p,
        Err(e) => { env.throw_new("java/lang/RuntimeException", e.to_string()).ok(); return; }
    };
    let out_cap = match env.get_direct_buffer_capacity(&out_buf) {
        Ok(c) => c,
        Err(e) => { env.throw_new("java/lang/RuntimeException", e.to_string()).ok(); return; }
    };

    let w = width as u32;
    let h = height as u32;
    let y_size  = (w * h) as usize;
    let uv_size = ((w / 2) * (h / 2)) as usize;

    if out_cap < y_size + uv_size * 2 {
        env.throw_new("java/lang/IllegalArgumentException", "Output buffer is too small!").ok();
        return;
    }

    let bgr_slice = unsafe { std::slice::from_raw_parts(bgr_ptr, bgr_cap) };
    let out_slice = unsafe { std::slice::from_raw_parts_mut(out_ptr, out_cap) };

    let (y_plane, rest)     = out_slice.split_at_mut(y_size);
    let (u_plane, v_plane)  = rest.split_at_mut(uv_size);

    let mut planar = YuvPlanarImageMut {
        y_plane:  BufferStoreMut::Borrowed(y_plane),
        u_plane:  BufferStoreMut::Borrowed(u_plane),
        v_plane:  BufferStoreMut::Borrowed(v_plane),
        y_stride: w,
        u_stride: w / 2,
        v_stride: w / 2,
        width:    w,
        height:   h,
    };

    if let Err(e) = yuv::bgr_to_yuv420(
        &mut planar,
        bgr_slice,
        stride as u32,
        YuvRange::Limited,
        YuvStandardMatrix::Bt601,
        YuvConversionMode::Balanced,
    ) {
        env.throw_new("java/lang/RuntimeException", e.to_string()).ok();
    }
}
