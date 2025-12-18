package tej.imageconverter;

public class ImageConverter {
    static {
        System.loadLibrary("imageconverter_rs"); // no lib prefix on Windows
    }

    public static native String convert(int a, int b);
}
