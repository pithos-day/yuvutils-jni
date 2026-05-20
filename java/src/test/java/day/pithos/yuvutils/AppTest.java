package day.pithos.yuvutils;

import junit.framework.TestCase;

public class AppTest extends TestCase {
    // ---- 定数 ----
    private static final int WIDTH  = 4;
    private static final int HEIGHT = 4;

    // ---- ヘルパー ----

    /** 単色 RGB 画像バッファを生成する */
    private static byte[] makeSolidRgb(int width, int height, int r, int g, int b) {
        byte[] buf = new byte[width * height * 3];
        for (int i = 0; i < width * height; i++) {
            buf[i * 3]     = (byte) r;
            buf[i * 3 + 1] = (byte) g;
            buf[i * 3 + 2] = (byte) b;
        }
        return buf;
    }

    /** YUV420 の各プレーンサイズ */
    private static int ySize(int w, int h)  { return w * h; }
    private static int uvSize(int w, int h) { return (w / 2) * (h / 2); }

    // ---- テスト ----

    /** 戻り値が null でないこと */
    public void testReturnValueIsNotNull() {
        byte[] rgb    = makeSolidRgb(WIDTH, HEIGHT, 128, 64, 32);
        byte[] result = YuvUtils.rgbToYuv420(rgb, WIDTH, HEIGHT);
        assertNotNull("rgbToYuv420 が null を返した", result);
    }

    /** 出力バッファのサイズが Y + U + V = w*h + (w/2)*(h/2)*2 であること */
    public void testOutputBufferSize() {
        byte[] rgb      = makeSolidRgb(WIDTH, HEIGHT, 100, 100, 100);
        byte[] result   = YuvUtils.rgbToYuv420(rgb, WIDTH, HEIGHT);
        int    expected = ySize(WIDTH, HEIGHT) + uvSize(WIDTH, HEIGHT) * 2;
        assertEquals("出力バッファサイズが不正", expected, result.length);
    }

    /** 純白(255,255,255) の Y 値が Limited Range の最大に近いこと (Y >= 200) */
    public void testWhiteYPlaneIsHigh() {
        byte[] rgb    = makeSolidRgb(WIDTH, HEIGHT, 255, 255, 255);
        byte[] result = YuvUtils.rgbToYuv420(rgb, WIDTH, HEIGHT);
        int    yVal   = Byte.toUnsignedInt(result[0]);
        assertTrue("白のY値が低すぎる: " + yVal, yVal >= 200);
    }

    /** 純黒(0,0,0) の Y 値が Limited Range の最小に近いこと (Y <= 30) */
    public void testBlackYPlaneIsLow() {
        byte[] rgb    = makeSolidRgb(WIDTH, HEIGHT, 0, 0, 0);
        byte[] result = YuvUtils.rgbToYuv420(rgb, WIDTH, HEIGHT);
        int    yVal   = Byte.toUnsignedInt(result[0]);
        assertTrue("黒のY値が高すぎる: " + yVal, yVal <= 30);
    }

    /** 単色画像では UV プレーンが全て同じ値であること */
    public void testSolidColorUvPlanesAreUniform() {
        byte[] rgb    = makeSolidRgb(WIDTH, HEIGHT, 200, 100, 50);
        byte[] result = YuvUtils.rgbToYuv420(rgb, WIDTH, HEIGHT);

        int yEnd  = ySize(WIDTH, HEIGHT);
        int uvLen = uvSize(WIDTH, HEIGHT);

        // U プレーン
        for (int i = yEnd + 1; i < yEnd + uvLen; i++) {
            assertEquals("Uプレーンの値が不均一 (index=" + i + ")", result[yEnd], result[i]);
        }
        // V プレーン
        int vStart = yEnd + uvLen;
        for (int i = vStart + 1; i < vStart + uvLen; i++) {
            assertEquals("Vプレーンの値が不均一 (index=" + i + ")", result[vStart], result[i]);
        }
    }

    /** グレー(R==G==B) では UV がほぼ 128 (無彩色) であること */
    public void testGrayImageHasNeutralUV() {
        byte[] rgb    = makeSolidRgb(WIDTH, HEIGHT, 128, 128, 128);
        byte[] result = YuvUtils.rgbToYuv420(rgb, WIDTH, HEIGHT);

        int yEnd  = ySize(WIDTH, HEIGHT);
        int uvLen = uvSize(WIDTH, HEIGHT);

        int uVal = Byte.toUnsignedInt(result[yEnd]);
        int vVal = Byte.toUnsignedInt(result[yEnd + uvLen]);

        // Limited Range の中点は 128 ± 10 以内
        assertTrue("グレーのU値が中点から外れすぎ: " + uVal, Math.abs(uVal - 128) <= 10);
        assertTrue("グレーのV値が中点から外れすぎ: " + vVal, Math.abs(vVal - 128) <= 10);
    }

    /** 入力バッファサイズが不正(短すぎる)でも例外か null になること */
    public void testInvalidInputBufferDoesNotCrash() {
        byte[] tooSmall = new byte[1]; // 明らかに短い
        try {
            byte[] result = YuvUtils.rgbToYuv420(tooSmall, WIDTH, HEIGHT);
            // null が返ってきた場合も許容(Rust 側で null_mut() を返す実装)
            assertNull("不正入力で null 以外が返った", result);
        } catch (RuntimeException e) {
            // Rust 側が throw_new した場合もOK
        }
    }
}