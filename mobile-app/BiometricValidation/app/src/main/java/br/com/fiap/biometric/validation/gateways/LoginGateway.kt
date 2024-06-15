package br.com.fiap.biometric.validation.gateways

import retrofit2.Call
import retrofit2.http.Body
import retrofit2.http.Headers
import retrofit2.http.POST
import java.util.UUID

data class LoginRequest(
    val email: String = "",
    val password: String = ""
)

data class LoginResponse(
    val id: UUID,
    val email: String
)

interface LoginGateway {
    @Headers("Content-Type: application/json")
    @POST("/auth/singin")
    fun logar(@Body request: LoginRequest): Call<LoginResponse>

    @Headers("Content-Type: application/json")
    @POST("/auth/singup")
    fun cadastrar(@Body request: LoginRequest): Call<LoginResponse>
}