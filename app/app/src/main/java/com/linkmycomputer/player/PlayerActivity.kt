package com.linkmycomputer.player

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity

class PlayerActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val touchOverlayView = TouchOverlayView(this).apply {
            onTouchFrame = { frame ->
                // TODO: forward to WebRTC data channel once signaling is wired.
                frame
            }
        }

        setContentView(touchOverlayView)
    }
}
