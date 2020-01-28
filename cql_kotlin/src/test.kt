package tests;

import java.nio.file.Path;
import java.nio.file.Paths;
import kotlinx.cinterop.*;

enum class StorageType(val rgb: Int) {
    F64Nullable(0),
    TinyText(1),
}

@CStruct("AxisDefinition") data class AxisDefinition(val id: Long, val max: Long);

class Test() {
    external fun hello(storageType: Int, axis: AxisDefinition): Int;

    fun main() {
        val p = Paths.get("target/debug/libcql_kotlin.so");
        System.load(p.toAbsolutePath().toString());
        val x = hello(StorageType.TinyText.ordinal, AxisDefinition(2, 3));
        println(StorageType.values()[x].name);
    }
}

fun main(args: Array<String>) {
    Test().main();
}
