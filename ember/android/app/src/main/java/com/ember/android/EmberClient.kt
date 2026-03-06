package com.ember.android

object EmberClient {
    init {
        System.loadLibrary("ember_native")
    }

    external fun connect(serverAddr: String): String
}
