package com.linkmycomputer.player

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TouchTrackerTest {

    @Test
    fun downMoveUp_keepsPointerIdAndActionOrder() {
        val tracker = TouchTracker()

        val down = tracker.onDown(pointerId = 2, x = 0.1f, y = 0.2f, pressure = 0.8f, timestampMs = 10)
        val move = tracker.onMove(
            listOf(PointerInput(pointerId = 2, x = 0.3f, y = 0.4f, pressure = 0.7f)),
            timestampMs = 20
        )
        val up = tracker.onUp(pointerId = 2, x = 0.3f, y = 0.4f, pressure = 0.1f, timestampMs = 30)

        assertEquals(PointerAction.DOWN, down.events.single().action)
        assertEquals(PointerAction.MOVE, move.events.single().action)
        assertEquals(PointerAction.UP, up.events.single().action)
        assertEquals(2, down.events.single().pointerId)
        assertEquals(2, move.events.single().pointerId)
        assertEquals(2, up.events.single().pointerId)
        assertEquals(0, tracker.activePointerCount())
    }

    @Test
    fun coordinatesAndPressureAreClampedToUnitRange() {
        val tracker = TouchTracker()
        val frame = tracker.onDown(pointerId = 1, x = 2.4f, y = -1.0f, pressure = 9.0f, timestampMs = 10)
        val snapshot = frame.events.single()

        assertEquals(1.0f, snapshot.x)
        assertEquals(0.0f, snapshot.y)
        assertEquals(1.0f, snapshot.pressure)
    }

    @Test
    fun cancel_removesPointerFromActiveSet() {
        val tracker = TouchTracker()
        tracker.onDown(pointerId = 9, x = 0.5f, y = 0.5f, pressure = 0.5f, timestampMs = 10)
        val cancel = tracker.onCancel(pointerId = 9, timestampMs = 12)

        assertEquals(PointerAction.CANCEL, cancel.events.single().action)
        assertTrue(tracker.activePointerCount() == 0)
    }
}
