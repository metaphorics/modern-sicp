// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch5

import java.io.File

/**
 * The edition-owned native oracle launcher (grammar section 8): it resolves
 * the pinned Kotlin 2.4.20 compiler jars from the configured Gradle cache at
 * runtime instead of hardcoding user-machine cache paths, then drives the
 * recorded oracle argv (K2JVMCompiler, -no-stdlib, -jvm-target 25). No
 * system `kotlinc` is consulted anywhere on this path.
 */
internal object NativeOracle {
    private const val KOTLIN_VERSION = "2.4.20"

    fun compile(
        source: String,
        work: String,
    ): Int {
        val resolved = resolve() ?: return 1
        val out = File(work, "classes")
        out.mkdirs()
        return process(
            listOf(
                "java",
                "-cp",
                resolved.compilerClasspath,
                "org.jetbrains.kotlin.cli.jvm.K2JVMCompiler",
                "-no-stdlib",
                "-classpath",
                resolved.stdlib,
                "-jvm-target",
                "25",
                "-d",
                out.absolutePath,
                source,
            ),
        )
    }

    fun run(work: String): Int {
        val resolved = resolve() ?: return 1
        return process(
            listOf("java", "-cp", File(work, "classes").absolutePath + File.pathSeparator + resolved.stdlib, "ProgramKt"),
        )
    }

    private class Resolved(
        val compilerClasspath: String,
        val stdlib: String,
    )

    private fun resolve(): Resolved? {
        val cache = File(System.getProperty("user.home"), ".gradle/caches/modules-2/files-2.1")
        if (!cache.isDirectory) {
            System.err.println("native oracle: Gradle cache not found at ${cache.absolutePath}")
            return null
        }
        val pinned =
            listOf(
                Triple("org.jetbrains.kotlin", "kotlin-compiler-embeddable", KOTLIN_VERSION),
                Triple("org.jetbrains.kotlin", "kotlin-build-tools-api", KOTLIN_VERSION),
                Triple("org.jetbrains.kotlin", "kotlin-stdlib", KOTLIN_VERSION),
                Triple("org.jetbrains.kotlin", "kotlin-script-runtime", KOTLIN_VERSION),
                Triple("org.jetbrains.kotlin", "kotlin-reflect", "1.6.10"),
                Triple("org.jetbrains.kotlin", "kotlin-daemon-embeddable", KOTLIN_VERSION),
                Triple("org.jetbrains.kotlinx", "kotlinx-coroutines-core-jvm", "1.8.0"),
                Triple("org.jetbrains", "annotations", "23.0.0"),
            )
        val jars =
            pinned.map { artifact ->
                findJar(cache, artifact.first, artifact.second, artifact.third) ?: run {
                    System.err.println(
                        "native oracle: missing pinned artifact ${artifact.first}:${artifact.second}:${artifact.third} under ${cache.absolutePath}",
                    )
                    return null
                }
            }
        return Resolved(jars.joinToString(File.pathSeparator) { it.absolutePath }, jars[2].absolutePath)
    }

    private fun findJar(
        cache: File,
        group: String,
        artifact: String,
        version: String,
    ): File? {
        val directory = File(cache, "$group/$artifact/$version")
        if (!directory.isDirectory) return null
        return directory.walkTopDown().firstOrNull { file ->
            file.isFile && file.name == "$artifact-$version.jar"
        }
    }

    private fun process(argv: List<String>): Int =
        try {
            ProcessBuilder(argv).inheritIO().start().waitFor()
        } catch (failure: Exception) {
            System.err.println("native oracle: cannot start java: ${failure.message}")
            1
        }
}
