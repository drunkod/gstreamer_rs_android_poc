package com.mozilla.gstgreetings;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.widget.TextView;

public class GreetingsActivity extends AppCompatActivity {

    static {
        System.loadLibrary("gstreamer_android");
        System.loadLibrary("greetings");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_greetings);

        RustGstGreetings g = new RustGstGreetings();
        String r = g.sayHello();
        ((TextView)findViewById(R.id.greetingField)).setText(r);
    }
}
