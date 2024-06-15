package br.com.fiap.biometric.validation.screens

import android.annotation.SuppressLint
import android.content.Context
import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Card
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
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
fun CadastroScreen(context: Context, navController: NavController) {
    val scrollState = rememberScrollState()
    var email by remember { mutableStateOf("") }
    var senha by remember { mutableStateOf("") }
    var confSenha by remember { mutableStateOf("") }
    Scaffold { innerPadding ->
        Column(
            modifier = Modifier
                .padding(16.dp)
                .fillMaxWidth()
                .verticalScroll(scrollState)
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
                TextField(
                    value = senha,
                    onValueChange = { senha = it },
                    label = { Text("Senha") },
                    modifier = Modifier.fillMaxWidth().padding(8.dp)
                )
                TextField(
                    value = confSenha,
                    onValueChange = { confSenha = it },
                    label = { Text("Confirmar Senha") },
                    modifier = Modifier.fillMaxWidth().padding(8.dp)
                )
                OutlinedButton(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 8.dp),
                    onClick = {
                        if (senha == confSenha) {
                            val request = LoginRequest(email, senha)
                            val client = HttpClient().createLoginGateway().cadastrar(request)
                            client.enqueue(object: Callback<LoginResponse> {
                                override fun onResponse(
                                    call: Call<LoginResponse>,
                                    response: Response<LoginResponse>
                                ) {
                                    AlermsToats().showMessage(context, "Cadastro realizado com sucesso!")
                                    navController.navigate("loginScreen")
                                }

                                override fun onFailure(call: Call<LoginResponse>, t: Throwable) {
                                    AlermsToats().showMessage(context, "Error ao tentar fazer cadastro, tente novamente!")
                                    Log.e("App", "Falha no cadastro: ${t.message}")
                                }

                            })
                        } else {
                            AlermsToats().showMessage(context, "Senhas não são iguais!")
                        }
                    }
                ) {
                    Text("Cadastrar e voltar ao Login!")
                }
            }
        }
    }
}
@Preview(showBackground = true)
@Composable
fun PreviewCadastroScreen() {
    var navController = rememberNavController()
    CadastroScreen(LocalContext.current, navController)
}