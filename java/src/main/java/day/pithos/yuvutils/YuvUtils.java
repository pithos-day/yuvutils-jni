package day.pithos.yuvutils;

import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

public class YuvUtils {

    static {
        load();
    }

    private static void load() {
        try {
            String libName = "libyuvutils_jni.so";

            InputStream in =
                    YuvUtils.class.getResourceAsStream(
                            "/natives/linux-x86_64/" + libName
                    );

            if (in == null) {
                throw new RuntimeException("Native library not found");
            }

            Path temp = Files.createTempFile(
                    "yuvutils",
                    ".so"
            );

            Files.copy(
                    in,
                    temp,
                    StandardCopyOption.REPLACE_EXISTING
            );

            System.load(temp.toAbsolutePath().toString());

        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public static native byte[] rgbToYuv420(byte[] rgb, int width, int height);
}
