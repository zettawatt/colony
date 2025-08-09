package com.colony.gui

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.webkit.MimeTypeMap
import androidx.core.content.FileProvider
import java.io.File

class FileOpener {
    companion object {
        @JvmStatic
        fun openFileWithDefaultApp(activity: Activity, filePath: String): String {
            return try {
                val file = File(filePath)
                if (!file.exists()) {
                    return "Error: File does not exist: $filePath"
                }

                val mimeType = getMimeType(file.absolutePath) ?: "*/*"
                val authority = "${activity.packageName}.fileprovider"

                val contentUri = try {
                    FileProvider.getUriForFile(activity, authority, file)
                } catch (e: IllegalArgumentException) {
                    Uri.fromFile(file)
                }

                val intent = Intent(Intent.ACTION_VIEW).apply {
                    setDataAndType(contentUri, mimeType)
                    addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
                    addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                }

                val resolveInfo = intent.resolveActivity(activity.packageManager)
                if (resolveInfo != null) {
                    activity.startActivity(intent)
                    "File opened successfully"
                } else {
                    val chooserIntent = Intent.createChooser(intent, "Open file with...")
                    val chooserResolveInfo = chooserIntent.resolveActivity(activity.packageManager)
                    if (chooserResolveInfo != null) {
                        activity.startActivity(chooserIntent)
                        "File opened with chooser"
                    } else {
                        "Error: No application found to open this file type"
                    }
                }
            } catch (e: Exception) {
                "Error: Failed to open file: ${e.message}"
            }
        }

        @JvmStatic
        private fun getMimeType(filePath: String): String? {
            val extension = MimeTypeMap.getFileExtensionFromUrl(filePath)
            return if (extension != null) {
                MimeTypeMap.getSingleton().getMimeTypeFromExtension(extension.lowercase())
            } else {
                null
            }
        }
    }
}
