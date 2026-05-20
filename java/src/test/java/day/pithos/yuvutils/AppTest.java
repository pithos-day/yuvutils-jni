package day.pithos.yuvutils;

import junit.framework.TestCase;

public class AppTest extends TestCase {

    public void testLength() {
        byte[] data = new byte[123];

        int len = YUVUtils.getLength(data);

        System.out.println(len);
    }
}
