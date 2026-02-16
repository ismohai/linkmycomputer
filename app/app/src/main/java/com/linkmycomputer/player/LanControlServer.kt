package com.linkmycomputer.player

import android.os.Build
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.net.InetSocketAddress
import java.net.SocketException
import java.util.concurrent.atomic.AtomicBoolean

data class ConnectRequest(
    val desktopName: String,
    val hostAddress: String,
    val hostPort: Int
)

data class LanConnectionState(
    val connected: Boolean,
    val desktopName: String? = null,
    val hostAddress: String? = null
)

class LanControlServer(
    private val onStatus: (String) -> Unit,
    private val onConnectRequest: (ConnectRequest, (Boolean) -> Unit) -> Unit,
    private val onConnectionChanged: (LanConnectionState) -> Unit
) {
    private val running = AtomicBoolean(false)
    private var discoverySocket: DatagramSocket? = null
    private var controlSocket: DatagramSocket? = null
    private var discoveryThread: Thread? = null
    private var controlThread: Thread? = null

    @Volatile
    private var connectedEndpoint: InetSocketAddress? = null

    @Volatile
    private var connectedDesktopName: String? = null

    fun start() {
        if (!running.compareAndSet(false, true)) {
            return
        }

        startDiscoveryLoop()
        startControlLoop()
        onStatus("已启动局域网监听，等待电脑扫描。")
    }

    fun stop() {
        if (!running.compareAndSet(true, false)) {
            return
        }

        notifyDesktop("LMC_DISCONNECTED|APP_EXIT")

        try {
            discoverySocket?.close()
            controlSocket?.close()
        } catch (_: Exception) {
        }

        discoveryThread = null
        controlThread = null
        connectedEndpoint = null
        connectedDesktopName = null
        onConnectionChanged(LanConnectionState(connected = false))
    }

    fun disconnectFromDesktop() {
        notifyDesktop("LMC_DISCONNECTED|PHONE_MANUAL")
        connectedEndpoint = null
        connectedDesktopName = null
        onConnectionChanged(LanConnectionState(connected = false))
        onStatus("已从手机端主动断开连接。")
    }

    private fun startDiscoveryLoop() {
        discoveryThread = Thread {
            try {
                val socket = DatagramSocket(DISCOVERY_PORT)
                socket.broadcast = true
                socket.soTimeout = SOCKET_TIMEOUT_MS
                discoverySocket = socket

                val buffer = ByteArray(1024)
                while (running.get()) {
                    try {
                        val packet = DatagramPacket(buffer, buffer.size)
                        socket.receive(packet)

                        val message = String(packet.data, 0, packet.length)
                        if (message.startsWith("LMC_DISCOVER|")) {
                            val replyPort = parseReplyPort(message, packet.port)
                            val response =
                                "LMC_DEVICE|${Build.MODEL}|$APP_VERSION|$CONTROL_PORT"
                            sendPacket(response, packet.address, replyPort)
                        }
                    } catch (_: java.net.SocketTimeoutException) {
                    } catch (err: SocketException) {
                        if (!running.get()) {
                            break
                        }
                        onStatus("扫描监听异常：${err.message}")
                    }
                }
            } catch (err: Exception) {
                onStatus("扫描监听启动失败：${err.message}")
            }
        }.apply {
            name = "LanDiscoveryThread"
            start()
        }
    }

    private fun startControlLoop() {
        controlThread = Thread {
            try {
                val socket = DatagramSocket(CONTROL_PORT)
                socket.soTimeout = SOCKET_TIMEOUT_MS
                controlSocket = socket

                val buffer = ByteArray(1024)
                while (running.get()) {
                    try {
                        val packet = DatagramPacket(buffer, buffer.size)
                        socket.receive(packet)

                        val message = String(packet.data, 0, packet.length)
                        handleControlMessage(message, packet.address, packet.port)
                    } catch (_: java.net.SocketTimeoutException) {
                    } catch (err: SocketException) {
                        if (!running.get()) {
                            break
                        }
                        onStatus("控制监听异常：${err.message}")
                    }
                }
            } catch (err: Exception) {
                onStatus("控制监听启动失败：${err.message}")
            }
        }.apply {
            name = "LanControlThread"
            start()
        }
    }

    private fun handleControlMessage(message: String, address: InetAddress, port: Int) {
        when {
            message.startsWith("LMC_CONNECT_REQUEST|") -> {
                val desktopName = message.split("|").getOrNull(1)?.ifBlank { "未知电脑" } ?: "未知电脑"
                val request = ConnectRequest(
                    desktopName = desktopName,
                    hostAddress = address.hostAddress ?: "未知地址",
                    hostPort = port
                )

                onConnectRequest(request) { accepted ->
                    if (accepted) {
                        connectedEndpoint = InetSocketAddress(address, port)
                        connectedDesktopName = desktopName
                        sendPacket("LMC_CONNECT_ACCEPT|${Build.MODEL}", address, port)
                        onConnectionChanged(
                            LanConnectionState(
                                connected = true,
                                desktopName = desktopName,
                                hostAddress = address.hostAddress
                            )
                        )
                        onStatus("已连接电脑：$desktopName")
                    } else {
                        sendPacket("LMC_CONNECT_REJECT|PHONE_REJECT", address, port)
                        onStatus("已拒绝来自 $desktopName 的连接请求。")
                    }
                }
            }

            message.startsWith("LMC_DISCONNECT") -> {
                connectedEndpoint = null
                connectedDesktopName = null
                onConnectionChanged(LanConnectionState(connected = false))
                onStatus("电脑端已断开连接。")
            }

            message == "LMC_PING" -> {
                sendPacket("LMC_PONG", address, port)
            }
        }
    }

    private fun notifyDesktop(message: String) {
        val endpoint = connectedEndpoint ?: return
        sendPacket(message, endpoint.address, endpoint.port)
    }

    private fun sendPacket(message: String, address: InetAddress, port: Int) {
        try {
            DatagramSocket().use { socket ->
                val payload = message.toByteArray(Charsets.UTF_8)
                val packet = DatagramPacket(payload, payload.size, address, port)
                socket.send(packet)
            }
        } catch (_: Exception) {
        }
    }

    private fun parseReplyPort(message: String, fallback: Int): Int {
        val parts = message.split("|")
        return parts.getOrNull(2)?.toIntOrNull() ?: fallback
    }

    companion object {
        const val DISCOVERY_PORT = 42042
        const val CONTROL_PORT = 42043
        private const val SOCKET_TIMEOUT_MS = 700
        private const val APP_VERSION = "0.1.0"
    }
}
