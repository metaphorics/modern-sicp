// SPDX-License-Identifier: MIT

import org.jetbrains.kotlin.gradle.dsl.KotlinJvmProjectExtension

configure<KotlinJvmProjectExtension> {
    explicitApi()
}

dependencies {
    "api"(libs.arrow.core)
    "api"(libs.immutable)
}
