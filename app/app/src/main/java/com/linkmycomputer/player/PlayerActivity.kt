package com.linkmycomputer.player

import android.graphics.Color
import android.os.Bundle
import android.util.TypedValue
import android.view.Gravity
import android.view.View
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity

class PlayerActivity : AppCompatActivity() {
    private lateinit var statusText: TextView
    private lateinit var connectionText: TextView
    private lateinit var disconnectButton: Button
    private lateinit var lanServer: LanControlServer

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(createContentView())

        lanServer = LanControlServer(
            onStatus = { message ->
                runOnUiThread {
                    statusText.text = "状态：$message"
                }
            },
            onConnectRequest = { request, reply ->
                runOnUiThread {
                    showConnectDialog(request, reply)
                }
            },
            onConnectionChanged = { state ->
                runOnUiThread {
                    updateConnectionState(state)
                }
            }
        )

        disconnectButton.setOnClickListener {
            lanServer.disconnectFromDesktop()
        }

        lanServer.start()
    }

    override fun onDestroy() {
        if (::lanServer.isInitialized) {
            lanServer.stop()
        }
        super.onDestroy()
    }

    private fun createContentView(): View {
        val root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(Color.parseColor("#F4EFE4"))
            setPadding(dp(20), dp(24), dp(20), dp(24))
            gravity = Gravity.TOP
        }

        val title = TextView(this).apply {
            text = "LinkMyComputer 手机端"
            setTextColor(Color.parseColor("#1F2A2A"))
            setTextSize(TypedValue.COMPLEX_UNIT_SP, 24f)
            setTypeface(typeface, android.graphics.Typeface.BOLD)
        }

        val subtitle = TextView(this).apply {
            text = "保持本应用前台运行，等待电脑端扫描并发起连接请求。"
            setTextColor(Color.parseColor("#465252"))
            setTextSize(TypedValue.COMPLEX_UNIT_SP, 14f)
            setPadding(0, dp(10), 0, dp(16))
        }

        statusText = TextView(this).apply {
            text = "状态：正在初始化..."
            setTextColor(Color.parseColor("#384242"))
            setTextSize(TypedValue.COMPLEX_UNIT_SP, 15f)
            setPadding(0, 0, 0, dp(8))
        }

        connectionText = TextView(this).apply {
            text = "连接：未连接电脑"
            setTextColor(Color.parseColor("#384242"))
            setTextSize(TypedValue.COMPLEX_UNIT_SP, 15f)
            setPadding(0, 0, 0, dp(18))
        }

        disconnectButton = Button(this).apply {
            text = "断开当前连接"
            isEnabled = false
        }

        root.addView(title)
        root.addView(subtitle)
        root.addView(statusText)
        root.addView(connectionText)
        root.addView(disconnectButton)

        return root
    }

    private fun showConnectDialog(request: ConnectRequest, reply: (Boolean) -> Unit) {
        AlertDialog.Builder(this)
            .setTitle("连接请求")
            .setMessage("电脑 ${request.desktopName}（${request.hostAddress}）请求连接手机，是否允许？")
            .setCancelable(false)
            .setPositiveButton("允许") { _, _ ->
                reply(true)
            }
            .setNegativeButton("拒绝") { _, _ ->
                reply(false)
            }
            .show()
    }

    private fun updateConnectionState(state: LanConnectionState) {
        if (state.connected) {
            connectionText.text = "连接：已连接 ${state.desktopName ?: "未知电脑"}（${state.hostAddress ?: "未知地址"}）"
            disconnectButton.isEnabled = true
        } else {
            connectionText.text = "连接：未连接电脑"
            disconnectButton.isEnabled = false
        }
    }

    private fun dp(value: Int): Int {
        return (value * resources.displayMetrics.density).toInt()
    }
}
