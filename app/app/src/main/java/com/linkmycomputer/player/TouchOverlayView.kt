package com.linkmycomputer.player

import android.content.Context
import android.util.AttributeSet
import android.view.MotionEvent
import android.view.View

class TouchOverlayView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null
) : View(context, attrs) {

    private val tracker = TouchTracker()
    var onTouchFrame: ((TouchFrame) -> Unit)? = null

    override fun onTouchEvent(event: MotionEvent): Boolean {
        val now = event.eventTime
        when (event.actionMasked) {
            MotionEvent.ACTION_DOWN,
            MotionEvent.ACTION_POINTER_DOWN -> {
                val index = event.actionIndex
                val frame = tracker.onDown(
                    pointerId = event.getPointerId(index),
                    x = normalizeX(event.getX(index)),
                    y = normalizeY(event.getY(index)),
                    pressure = event.getPressure(index),
                    timestampMs = now
                )
                onTouchFrame?.invoke(frame)
            }

            MotionEvent.ACTION_MOVE -> {
                val inputs = (0 until event.pointerCount).map { i ->
                    PointerInput(
                        pointerId = event.getPointerId(i),
                        x = normalizeX(event.getX(i)),
                        y = normalizeY(event.getY(i)),
                        pressure = event.getPressure(i)
                    )
                }
                onTouchFrame?.invoke(tracker.onMove(inputs, now))
            }

            MotionEvent.ACTION_UP,
            MotionEvent.ACTION_POINTER_UP -> {
                val index = event.actionIndex
                val frame = tracker.onUp(
                    pointerId = event.getPointerId(index),
                    x = normalizeX(event.getX(index)),
                    y = normalizeY(event.getY(index)),
                    pressure = event.getPressure(index),
                    timestampMs = now
                )
                onTouchFrame?.invoke(frame)
            }

            MotionEvent.ACTION_CANCEL -> {
                val ids = (0 until event.pointerCount).map { event.getPointerId(it) }
                ids.forEach { pointerId ->
                    onTouchFrame?.invoke(tracker.onCancel(pointerId, now))
                }
            }
        }
        return true
    }

    private fun normalizeX(x: Float): Float {
        val w = if (width <= 0) 1f else width.toFloat()
        return (x / w).coerceIn(0f, 1f)
    }

    private fun normalizeY(y: Float): Float {
        val h = if (height <= 0) 1f else height.toFloat()
        return (y / h).coerceIn(0f, 1f)
    }
}
