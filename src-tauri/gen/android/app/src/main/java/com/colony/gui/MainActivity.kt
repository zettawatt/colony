package com.colony.gui

import android.content.Intent
import android.os.Bundle
import java.io.File
import java.net.ServerSocket
import java.net.Socket

class MainActivity : TauriActivity() {

    private var socketServerThread: Thread? = null
    private var isServerRunning = false
    private var serverSocket: ServerSocket? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        startSocketServer()
    }

    override fun onDestroy() {
        super.onDestroy()

        stopSocketServer()
    }

    private fun startSocketServer() {
        isServerRunning = true
        socketServerThread = Thread {
            try {
                serverSocket = ServerSocket(8765)

                while (isServerRunning) {
                    try {
                        val clientSocket = serverSocket?.accept()
                        if (clientSocket != null) {
                            Thread {
                                handleSocketClient(clientSocket)
                            }.start()
                        }
                    } catch (e: Exception) {
                        if (isServerRunning) {
                            Thread.sleep(1000)
                        }
                    }
                }
            } catch (e: Exception) {
                // Socket server error - silently handle
            }
        }
        socketServerThread?.start()
    }

    private fun stopSocketServer() {
        isServerRunning = false
        try {
            serverSocket?.close()
        } catch (e: Exception) {
            // Ignore close errors
        }
        socketServerThread?.interrupt()
        socketServerThread = null
    }

    private fun handleSocketClient(clientSocket: Socket) {
        try {
            val inputStream = clientSocket.inputStream
            val outputStream = clientSocket.outputStream

            val buffer = ByteArray(1024)
            val bytesRead = inputStream.read(buffer)
            if (bytesRead > 0) {
                val filePath = String(buffer, 0, bytesRead).trim()
                val result = FileOpener.openFileWithDefaultApp(this, filePath)
                outputStream.write(result.toByteArray())
                outputStream.flush()
            }

        } catch (e: Exception) {
            try {
                val errorResponse = "Error: ${e.message}"
                clientSocket.outputStream.write(errorResponse.toByteArray())
                clientSocket.outputStream.flush()
            } catch (writeError: Exception) {
                // Ignore write errors
            }
        } finally {
            try {
                clientSocket.close()
            } catch (e: Exception) {
                // Ignore close errors
            }
        }
    }
}