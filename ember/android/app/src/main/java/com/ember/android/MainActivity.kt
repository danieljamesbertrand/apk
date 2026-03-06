package com.ember.android

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val serverInput = findViewById<EditText>(R.id.server_address)
        val connectBtn = findViewById<Button>(R.id.connect_btn)
        val resultText = findViewById<TextView>(R.id.result_text)

        serverInput.setText("192.168.1.100:4433", TextView.BufferType.EDITABLE)

        connectBtn.setOnClickListener {
            val addr = serverInput.text.toString().trim()
            if (addr.isEmpty()) {
                Toast.makeText(this, "Enter server address", Toast.LENGTH_SHORT).show()
                return@setOnClickListener
            }

            connectBtn.isEnabled = false
            resultText.text = "Connecting..."

            lifecycleScope.launch {
                val result = withContext(Dispatchers.IO) {
                    try {
                        EmberClient.connect(addr)
                    } catch (e: Exception) {
                        "Error: ${e.message}"
                    }
                }
                resultText.text = result
                connectBtn.isEnabled = true
            }
        }
    }
}
