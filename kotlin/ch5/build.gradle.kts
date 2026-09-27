// SPDX-License-Identifier: MIT

import org.jetbrains.kotlin.gradle.dsl.KotlinJvmProjectExtension

// A chapter unit of decision 0001: the three root-level code directories are
// this project's examples, exercises, and solutions source sets. `check`
// compiles all three (D28: exercises are compile-only) and runs the examples
// and solutions tests; `exercisesTest` runs only from `just scaffold`.

val chapter: String = project.name

val examplesSet = sourceSets.create("examples")
val exercisesSet = sourceSets.create("exercises")
val solutionsSet = sourceSets.create("solutions")

configure<KotlinJvmProjectExtension> {
    sourceSets {
        getByName("examples") { kotlin.srcDir("../examples/$chapter") }
        getByName("exercises") { kotlin.srcDir("../exercises/$chapter") }
        getByName("solutions") { kotlin.srcDir("../solutions/$chapter") }
    }
}

// Section 5.2's simulator library (the machine model, the assembler, and the
// execution procedures of 5.2.1 to 5.2.3) lives in this project's `main`
// source set under sicp.ch5, the chapter-crate equivalent of the sibling
// editions. The three root-level source sets compile against it.
// Section 5.4's explicit-control evaluator needs 5.2 and 4.1 (the plan's
// dependency line): the chapter 4 reader reads the object-language source
// the controller's `read` operation hands to `eval-dispatch`.
val libraryOutput = sourceSets.getByName("main").output

dependencies {
    "implementation"(project(":runtime"))
    "implementation"(project(":ch4"))
    "examplesImplementation"(libraryOutput)
    "exercisesImplementation"(libraryOutput)
    "solutionsImplementation"(libraryOutput)
    "examplesImplementation"(project(":runtime"))
    "examplesImplementation"(project(":ch4"))
    "examplesImplementation"(libs.immutable)
    "examplesImplementation"(libs.kotest.runner.junit5)
    "examplesImplementation"(libs.kotest.assertions.core)
    "examplesImplementation"(libs.kotest.property)
    "exercisesImplementation"(project(":runtime"))
    "exercisesImplementation"(project(":ch4"))
    "exercisesImplementation"(libs.immutable)
    "exercisesImplementation"(libs.kotest.runner.junit5)
    "exercisesImplementation"(libs.kotest.assertions.core)
    "exercisesImplementation"(libs.kotest.property)
    "solutionsImplementation"(project(":runtime"))
    "solutionsImplementation"(project(":ch4"))
    "solutionsImplementation"(libs.immutable)
    "solutionsImplementation"(libs.kotest.runner.junit5)
    "solutionsImplementation"(libs.kotest.assertions.core)
    "solutionsImplementation"(libs.kotest.property)
}

val examplesTest =
    tasks.register<Test>("examplesTest") {
        description = "Runs the examples tests of this chapter."
        group = "verification"
        testClassesDirs = sourceSets.getByName("examples").output.classesDirs
        classpath = sourceSets.getByName("examples").runtimeClasspath
    }

val solutionsTest =
    tasks.register<Test>("solutionsTest") {
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
        sourceSets.getByName("main").output,
        sourceSets.getByName("examples").output,
        sourceSets.getByName("exercises").output,
        sourceSets.getByName("solutions").output,
    )
}
