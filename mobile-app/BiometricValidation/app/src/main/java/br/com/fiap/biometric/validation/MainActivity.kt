package br.com.fiap.biometric.validation

import android.Manifest
import android.annotation.SuppressLint
import android.content.Context
import android.net.Uri
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.core.content.FileProvider
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import br.com.fiap.biometric.validation.gateways.BiometricsRequest
import br.com.fiap.biometric.validation.gateways.BiometricsResponse
import br.com.fiap.biometric.validation.gateways.HttpClient
import br.com.fiap.biometric.validation.gateways.LoginRequest
import br.com.fiap.biometric.validation.gateways.LoginResponse
import br.com.fiap.biometric.validation.screens.CadastroScreen
import br.com.fiap.biometric.validation.screens.LoginScreen
import br.com.fiap.biometric.validation.ui.theme.BiometricValidationTheme
import br.com.fiap.biometric.validation.utils.AlermsToats
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response
import java.io.File
import java.util.Objects


class MainActivity : ComponentActivity() {
    private val context = this
    private val requestPermissionLauncher =
        registerForActivityResult(ActivityResultContracts.RequestPermission()
        ) { isGranted: Boolean ->
            if (isGranted) {
                AlermsToats().showMessage(this, "Camera com permissão!")
            } else {
                AlermsToats().showMessage(this, "Você precisa da camera!")
            }
        }

    @SuppressLint("PrivateResource")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            BiometricValidationTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    var navController = rememberNavController()
                    NavHost(navController, startDestination = "permissionScreen") {
                        composable("permissionScreen") { PermissionScreen(navController) }
                        composable("loginScreen") { LoginScreen(context, navController) }
                        composable("cadastroScreen") { CadastroScreen(context, navController) }
                        composable("biometricScreen/{customerId}/{jwt}") {
                            val customerId = it.arguments?.getString("customerId")!!
                            val jwt = it.arguments?.getString("jwt")!!
                            BiometricScreen(context, navController, customerId, jwt)
                        }
                        composable("previewAndWarnScreen/{isValid}") {
                            val isValid = it.arguments?.getString("isValid")
                            PreviewAndWarnScreen(navController, isValid.equals("true"))
                        }
                    }
                }
            }
        }
    }


    @Composable
    fun PermissionScreen(
        navController: NavController,
    ) {
        LaunchedEffect(Unit) {
            requestPermissionLauncher.launch(Manifest.permission.CAMERA)
        }
        Column(
            modifier = Modifier.fillMaxSize(),
            verticalArrangement = Arrangement.SpaceEvenly,
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(text = "Ola! Somos seu amigo validador de Faces. :)")
                Text(text = "Vamos começar sua Biometria Facial?")
                Text(text = "primeiro faça login ou cadastro!")
            }
            Button(
                onClick = {
                    navController.navigate("loginScreen")
                },
            ) {
                Text(text = "OK! Vamos lá!")
            }
        }
    }

    @Composable
    fun BiometricScreen(context: Context, navController: NavController, customerId: String, jwt: String) {
        val file = File(
            LocalContext.current.getExternalFilesDir(null),
            "image.jpg"
        )
        val uri = FileProvider.getUriForFile(
            Objects.requireNonNull(this),
            this.packageName + ".provider", file
        )
        var capturedImageUri by remember {
            mutableStateOf<Uri>(Uri.EMPTY)
        }
        val cameraLauncher =
            rememberLauncherForActivityResult(ActivityResultContracts.TakePicture()) { result ->
                if (result) {
                    capturedImageUri = uri
                    val request = BiometricsRequest(customerId, capturedImageUri.toString())
                    val client = HttpClient().createBiometricsGateway().criar(jwt, request)
                    client.enqueue(object: Callback<BiometricsResponse> {
                        override fun onResponse(
                            call: Call<BiometricsResponse>,
                            response: Response<BiometricsResponse>
                        ) {
                            navController.navigate("previewAndWarnScreen/true")
                        }

                        override fun onFailure(call: Call<BiometricsResponse>, t: Throwable) {
                            navController.navigate("previewAndWarnScreen/false")
                            Log.e("App", "Falha no criar biometria: ${t.message}")
                        }

                    })
                } else {
                    navController.navigate("previewAndWarnScreen/false")
                    Log.e("CaptureImage", "Erro ao capturar imagem. Resultado: $result")
                }
            }

        Column(
            modifier = Modifier.fillMaxSize(),
            verticalArrangement = Arrangement.SpaceEvenly,
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(text = "Hora da biometria!")
                Button(onClick = { cameraLauncher.launch(uri)  }) {
                    Text(text = "Abrir camera")
                }
            }
        }
    }

    @Composable
    fun PreviewAndWarnScreen(navController: NavController, isValid: Boolean) {
        Column(
            modifier = Modifier.fillMaxSize(),
            verticalArrangement = Arrangement.SpaceEvenly,
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                if (isValid) {
                    Text(text = "Deu tudo certinho! Face validada com sucesso :) !")
                } else {
                    Text(text = "Infelizmente tivemos um erro na validação :( !")
                }
                Button(onClick = {  navController.navigate("biometricScreen") }) {
                    Text(text = "Quer fazer uma nova validação?")
                    Text(text = "Só clicar aqui!")
                }
            }
        }
    }
}