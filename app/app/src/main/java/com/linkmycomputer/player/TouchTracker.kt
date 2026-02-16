package com.linkmycomputer.player

data class PointerInput(
    val pointerId: Int,
    val x: Float,
    val y: Float,
    val pressure: Float
)

enum class PointerAction {
    DOWN,
    MOVE,
    UP,
    CANCEL
}

data class PointerSnapshot(
    val pointerId: Int,
    val action: PointerAction,
    val x: Float,
    val y: Float,
    val pressure: Float,
    val timestampMs: Long
)

data class TouchFrame(
    val frameId: Long,
    val events: List<PointerSnapshot>
)

class TouchTracker {
    private var frameCounter: Long = 0
    private val activePointers = linkedMapOf<Int, PointerInput>()

    fun onDown(pointerId: Int, x: Float, y: Float, pressure: Float, timestampMs: Long): TouchFrame {
        val normalized = normalize(pointerId, x, y, pressure)
        activePointers[pointerId] = normalized
        return frameOf(
            PointerSnapshot(
                pointerId = normalized.pointerId,
                action = PointerAction.DOWN,
                x = normalized.x,
                y = normalized.y,
                pressure = normalized.pressure,
                timestampMs = timestampMs
            )
        )
    }

    fun onMove(inputs: List<PointerInput>, timestampMs: Long): TouchFrame {
        val snapshots = inputs.map {
            val normalized = normalize(it.pointerId, it.x, it.y, it.pressure)
            activePointers[it.pointerId] = normalized
            PointerSnapshot(
                pointerId = normalized.pointerId,
                action = PointerAction.MOVE,
                x = normalized.x,
                y = normalized.y,
                pressure = normalized.pressure,
                timestampMs = timestampMs
            )
        }
        return frameOf(*snapshots.toTypedArray())
    }

    fun onUp(pointerId: Int, x: Float, y: Float, pressure: Float, timestampMs: Long): TouchFrame {
        val normalized = normalize(pointerId, x, y, pressure)
        activePointers.remove(pointerId)
        return frameOf(
            PointerSnapshot(
                pointerId = normalized.pointerId,
                action = PointerAction.UP,
                x = normalized.x,
                y = normalized.y,
                pressure = normalized.pressure,
                timestampMs = timestampMs
            )
        )
    }

    fun onCancel(pointerId: Int, timestampMs: Long): TouchFrame {
        activePointers.remove(pointerId)
        return frameOf(
            PointerSnapshot(
                pointerId = pointerId,
                action = PointerAction.CANCEL,
                x = 0f,
                y = 0f,
                pressure = 0f,
                timestampMs = timestampMs
            )
        )
    }

    fun activePointerCount(): Int = activePointers.size

    private fun frameOf(vararg snapshots: PointerSnapshot): TouchFrame {
        frameCounter += 1
        return TouchFrame(frameCounter, snapshots.asList())
    }

    private fun normalize(pointerId: Int, x: Float, y: Float, pressure: Float): PointerInput {
        return PointerInput(
            pointerId = pointerId,
            x = x.coerceIn(0f, 1f),
            y = y.coerceIn(0f, 1f),
            pressure = pressure.coerceIn(0f, 1f)
        )
    }
}
