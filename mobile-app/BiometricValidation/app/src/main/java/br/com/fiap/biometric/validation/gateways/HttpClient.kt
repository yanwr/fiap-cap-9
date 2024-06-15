package br.com.fiap.biometric.validation.gateways

import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory

class HttpClient {
    private val BASE_URL = "http://localhots:9095"

    private val retrofitClient = Retrofit
        .Builder()
        .baseUrl(BASE_URL)
        .addConverterFactory(GsonConverterFactory.create())
        .build()

    fun createLoginGateway(): LoginGateway {
        return retrofitClient.create(LoginGateway::class.java)
    }

    fun createBiometricsGateway(): BiometricsGateway {
        return retrofitClient.create(BiometricsGateway::class.java)
    }
}