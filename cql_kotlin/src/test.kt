package tests;

import java.nio.file.Path;
import java.nio.file.Paths;

class Test() {
    external fun hello();

    fun main() {
        val p = Paths.get("target/debug/libcql_kotlin.so");
        System.load(p.toAbsolutePath().toString());
        hello();
    }
}

fun main(args: Array<String>) {
    Test().main();
}
