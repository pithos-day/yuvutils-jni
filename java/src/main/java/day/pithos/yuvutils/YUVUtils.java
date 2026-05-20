package day.pithos.yuvutils;

public class YUVUtils {
    static {
        String path = System.getProperty("user.dir") + "/../rust/target/debug/libyuvutils_jni.so";
        System.load(path);
    }

    public static native int getLength(byte[] input);
}
