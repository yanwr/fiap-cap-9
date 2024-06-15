package br.com.fiap.biometric.validation.utils

import android.content.Context
import android.widget.Toast

class AlermsToats {
    fun showMessage(context: Context, message: String) {
        Toast.makeText(context, message, Toast.LENGTH_LONG).show()
    }
}