package com.mozilla.gstgreetings;

/**
 * Created by ferjm on 14/03/2018.
 */

public class RustGstGreetings {

    private static native String greeting();

    public String sayHello() {
        return greeting();
    }
}
