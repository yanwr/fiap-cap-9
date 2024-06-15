package br.com.fiap.biometric.validation.gateways

import retrofit2.Call
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.Header
import retrofit2.http.Headers
import retrofit2.http.POST
import retrofit2.http.Path

data class BiometricsRequest(
    val customerId: String = "",
    val imagePath: String = ""
)

data class BiometricsResponse(
    val customerId: String = "",
    val imagePath: String = "",
    val status: String = "",
    val createdAt: String = "",
    val updatedAt: String = ""
)

interface BiometricsGateway {
    @Headers("Content-Type: application/json")
    @POST("/biometrics/actions/create")
    fun criar(@Header("Authorization") authorization: String, @Body request: BiometricsRequest): Call<BiometricsResponse>

    @GET("/biometrics/actions/get/{userId}")
    fun ver(@Path("userId") userId: String): Call<BiometricsResponse>
}