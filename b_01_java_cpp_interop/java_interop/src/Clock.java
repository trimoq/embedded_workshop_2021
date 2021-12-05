class Clock {
    
    private static native String myGetTime(String reason);

    static {
        System.loadLibrary("java_interop");
    }

    public static void main(String[] args) {
        String outRust = Clock.myGetTime("printing");
        String outJava = "Java: "+System.currentTimeMillis();

        System.out.println(outRust);
        System.out.println(outJava);
    }
}