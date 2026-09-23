// SPDX-License-Identifier: MIT

import org.jetbrains.kotlin.gradle.dsl.KotlinJvmProjectExtension

// A chapter unit of decision 0001: the three root-level code directories are
// this project's examples, exercises, and solutions source sets. `check`
// compiles all three (D28: exercises are compile-only) and runs the examples
// and solutions tests; `exercisesTest` runs only from `just scaffold`.

val chapter: String = project.name

configure<KotlinJvmProjectExtension> {
    sourceSets {
        create("examples") {
            kotlin.srcDir("../examples/$chapter")
        }
        create("exercises") {
            kotlin.srcDir("../exercises/$chapter")
        }
        create("solutions") {
            kotlin.srcDir("../solutions/$chapter")
        }
    }
}

dependencies {
    "examplesImplementation"(project(":runtime"))
    "examplesImplementation"(libs.immutable)
    "examplesImplementation"(libs.kotest.runner.junit5)
    "examplesImplementation"(libs.kotest.assertions.core)
    "examplesImplementation"(libs.kotest.property)
    "exercisesImplementation"(project(":runtime"))
    "exercisesImplementation"(libs.immutable)
    "exercisesImplementation"(libs.kotest.runner.junit5)
    "exercisesImplementation"(libs.kotest.assertions.core)
    "exercisesImplementation"(libs.kotest.property)
    "solutionsImplementation"(project(":runtime"))
    "solutionsImplementation"(libs.immutable)
    "solutionsImplementation"(libs.kotest.runner.junit5)
    "solutionsImplementation"(libs.kotest.assertions.core)
    "solutionsImplementation"(libs.kotest.property)
}

val examplesTest = tasks.register<Test>("examplesTest") {
    description = "Runs the examples tests of this chapter."
    group = "verification"
    testClassesDirs = sourceSets.getByName("examples").output.classesDirs
    classpath = sourceSets.getByName("examples").runtimeClasspath
}

val solutionsTest = tasks.register<Test>("solutionsTest") {
    description = "Runs the solutions tests of this chapter."
    group = "verification"
    testClassesDirs = sourceSets.getByName("solutions").output.classesDirs
    classpath = sourceSets.getByName("solutions").runtimeClasspath
}

tasks.register<Test>("exercisesTest") {
    description = "Runs the exercises tests of this chapter; pending scaffolds are disabled tests."
    group = "verification"
    testClassesDirs = sourceSets.getByName("exercises").output.classesDirs
    classpath = sourceSets.getByName("exercises").runtimeClasspath
}

tasks.named("check") {
    dependsOn(examplesTest, solutionsTest)
    dependsOn(
        tasks.named("compileKotlin"),
        tasks.named("compileExamplesKotlin"),
        tasks.named("compileExercisesKotlin"),
        tasks.named("compileSolutionsKotlin"),
    )
}
