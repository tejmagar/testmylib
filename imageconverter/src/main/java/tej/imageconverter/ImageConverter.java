package tej.imageconverter;

public class ImageConverter {
    static {
        System.loadLibrary("imageconverter_rs");
    }

    public static native String convert(int a, int b);
}
