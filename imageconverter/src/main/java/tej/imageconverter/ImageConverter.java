package tej.imageconverter;

import java.nio.ByteBuffer;

public class ImageConverter {
    static {
        System.loadLibrary("imageconverter_rs");
    }

    public static native byte[] bytes_from_raw(int width, int height, int channels, int rowStride,
                                               ByteBuffer byteBuffer, String convertTo);
}
