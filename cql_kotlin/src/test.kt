package tests;

import java.nio.file.Path;
import java.nio.file.Paths;

enum class StorageType(val rgb: Int) {
    F64Nullable(0),
    TinyText(1),
}

class Test() {
    external fun hello(storageType: Int): Int;

    fun main() {
        val p = Paths.get("target/debug/libcql_kotlin.so");
        System.load(p.toAbsolutePath().toString());
        val x = hello(StorageType.TinyText.ordinal);
        println(StorageType.values()[x].name);
    }
}

fun main(args: Array<String>) {
    Test().main();
}
