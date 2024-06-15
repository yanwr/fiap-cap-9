package br.com.fiap.biometric.validation.screens

import android.annotation.SuppressLint
import android.content.Context
import android.util.Log
import android.widget.Toast
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import br.com.fiap.biometric.validation.gateways.HttpClient
import br.com.fiap.biometric.validation.gateways.LoginRequest
import br.com.fiap.biometric.validation.gateways.LoginResponse
import br.com.fiap.biometric.validation.utils.AlermsToats
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response

@SuppressLint("UnusedMaterial3ScaffoldPaddingParameter")
@Composable
fun LoginScreen(context: Context, navController: NavController) {
    var email by remember { mutableStateOf("") }
    var senha by remember { mutableStateOf("") }
    Scaffold { innerPadding ->
        Column(
            modifier = Modifier
                .fillMaxSize()

        ) {
            Spacer(modifier = Modifier.height(innerPadding.calculateTopPadding()))
            Card(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp)
            ) {
                OutlinedTextField(
                    value = email,
                    onValueChange = { email = it },
                    label = { Text("Email") },
                    modifier = Modifier.fillMaxWidth().padding(8.dp)
                )
                OutlinedTextField(
                    value = senha,
                    onValueChange = { senha = it },
                    label = { Text("Senha") },
                    modifier = Modifier.fillMaxWidth().padding(8.dp)
                )
                OutlinedButton(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 8.dp),
                    onClick = {
                        val request = LoginRequest(email, senha)
                        val client = HttpClient().createLoginGateway().logar(request)
                        client.enqueue(object: Callback<LoginResponse> {
                            override fun onResponse(
                                call: Call<LoginResponse>,
                                response: Response<LoginResponse>
                            ) {
                                val jwt = response.headers().get("Authorization")!!
                                val customerId = response.body()?.id!!
                                AlermsToats().showMessage(context, "Login realizado com sucesso!")
                                navController.navigate("biometricScreen/${customerId}/${jwt}")
                            }

                            override fun onFailure(call: Call<LoginResponse>, t: Throwable) {
                                AlermsToats().showMessage(context, "Error ao tentar fazer login, tente novamente!")
                                Log.e("App", "Falha no login: ${t.message}")
                            }

                        })
                    }
                ) {
                    Text("Logar")
                }
                OutlinedButton(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 8.dp),
                    onClick = { navController.navigate("cadastroScreen") }
                ) {
                    Text("Cadastrar")
                }
            }
        }
    }
}
@Preview(showBackground = true)
@Composable
fun PreviewLoginScreen() {
    var navController = rememberNavController()
    LoginScreen(LocalContext.current, navController)
}