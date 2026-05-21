package day.pithos.yuvutils;

import java.io.InputStream;
import java.nio.ByteBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

public class YuvUtils {

    static {
        load();
    }

    private static void load() {
        try {
            String os = System.getProperty("os.name").toLowerCase();

            String platform;
            String libName;

            if (os.contains("win")) {
                platform = "windows-x86_64";
                libName = "yuvutils_jni.dll";

            } else if (os.contains("mac")) {
                platform = "macos-aarch64";
                libName = "libyuvutils_jni.dylib";

            } else {
                platform = "linux-x86_64";
                libName = "libyuvutils_jni.so";
            }

            String resourcePath =
                    "/native/" + platform + "/" + libName;

            InputStream in = YuvUtils.class.getResourceAsStream(resourcePath);

            if (in == null) {
                throw new RuntimeException("Native library not found: " + resourcePath);
            }

            Path temp = Files.createTempFile("yuvutils-", libName);
            temp.toFile().deleteOnExit();
            Files.copy(in, temp, StandardCopyOption.REPLACE_EXISTING);
            System.load(temp.toAbsolutePath().toString());

        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public static native byte[] rgbToYuv420(byte[] rgb, int width, int height);

    public static native byte[] bgrToYuv420(byte[] rgb, int width, int height, int stride);

    public static native void bgrToYuv420Direct(ByteBuffer bgr, ByteBuffer out, int width, int height, int stride);

}
