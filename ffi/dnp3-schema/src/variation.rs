use crate::gv;
use oo_bindgen::model::{BackTraced, EnumHandle, LibraryBuilder};

pub(crate) fn define(lib: &mut LibraryBuilder) -> BackTraced<EnumHandle> {
    let variation = lib
        .define_enum("variation")?
        .push(gv(1, 0), "Binary Input - Default variation")?
        .push(gv(1, 1), "Binary Input - Packed format")?
        .push(gv(1, 2), "Binary Input - With flags")?
        .push(gv(2, 0), "Binary Input Event - Default variation")?
        .push(gv(2, 1), "Binary Input Event - Without time")?
        .push(gv(2, 2), "Binary Input Event - With absolute time")?
        .push(gv(2, 3), "Binary Input Event - With relative time")?
        .push(gv(3, 0), "Double-bit Binary Input - Default variation")?
        .push(gv(3, 1), "Double-bit Binary Input - Packed format")?
        .push(gv(3, 2), "Double-bit Binary Input - With flags")?
        .push(
            gv(4, 0),
            "Double-bit Binary Input Event - Default variation",
        )?
        .push(gv(4, 1), "Double-bit Binary Input Event - Without time")?
        .push(
            gv(4, 2),
            "Double-bit Binary Input Event - With absolute time",
        )?
        .push(
            gv(4, 3),
            "Double-bit Binary Input Event - With relative time",
        )?
        .push(gv(10, 0), "Binary Output - Default variation")?
        .push(gv(10, 1), "Binary Output - Packed format")?
        .push(gv(10, 2), "Binary Output - With flags")?
        .push(gv(11, 0), "Binary Output Event - Default variation")?
        .push(gv(11, 1), "Binary Output Event - Without time")?
        .push(gv(11, 2), "Binary Output Event - With time")?
        .push(
            gv(12, 0),
            "Binary Output Command - Control Relay Output Block",
        )?
        .push(gv(12, 1), "Binary Output Command - Pattern Control Block")?
        /* TODO
        .push(gv(13, 1), "Binary Output Command Event - Without time")?
        .push(gv(13, 2), "Binary Output Command Event - With time")?
         */
        .push(gv(20, 0), "Counter - Default variation")?
        .push(gv(20, 1), "Counter - 32-bit with flags")?
        .push(gv(20, 2), "Counter - 16-bit with flags")?
        .push(gv(20, 5), "Counter - 32-bit without flag")?
        .push(gv(20, 6), "Counter - 16-bit without flag")?
        .push(gv(21, 0), "Frozen Counter - Default variation")?
        .push(gv(21, 1), "Frozen Counter - 32-bit with flags")?
        .push(gv(21, 2), "Frozen Counter - 16-bit with flags")?
        .push(gv(21, 5), "Frozen Counter - 32-bit with flags and time")?
        .push(gv(21, 6), "Frozen Counter - 16-bit with flags and time")?
        .push(gv(21, 9), "Frozen Counter - 32-bit without flag")?
        .push(gv(21, 10), "Frozen Counter - 16-bit without flag")?
        .push(gv(22, 0), "Counter Event - Default variation")?
        .push(gv(22, 1), "Counter Event - 32-bit with flags")?
        .push(gv(22, 2), "Counter Event - 16-bit with flags")?
        .push(gv(22, 5), "Counter Event - 32-bit with flags and time")?
        .push(gv(22, 6), "Counter Event - 16-bit with flags and time")?
        .push(gv(23, 0), "Frozen Counter Event - Default variation")?
        .push(gv(23, 1), "Frozen Counter Event - 32-bit with flags")?
        .push(gv(23, 2), "Frozen Counter Event - 16-bit with flags")?
        .push(
            gv(23, 5),
            "Frozen Counter Event - 32-bit with flags and time",
        )?
        .push(
            gv(23, 6),
            "Frozen Counter Event - 16-bit with flags and time",
        )?
        .push(gv(30, 0), "Analog Input - Default variation")?
        .push(gv(30, 1), "Analog Input - 32-bit with flags")?
        .push(gv(30, 2), "Analog Input - 16-bit with flags")?
        .push(gv(30, 3), "Analog Input - 32-bit without flag")?
        .push(gv(30, 4), "Analog Input - 16-bit without flag")?
        .push(
            gv(30, 5),
            "Analog Input - Single-precision floating point with flags",
        )?
        .push(
            gv(30, 6),
            "Analog Input - Double-precision floating point with flags",
        )?
        .push(gv(32, 0), "Analog Input Event - Default variation")?
        .push(gv(32, 1), "Analog Input Event - 32-bit without time")?
        .push(gv(32, 2), "Analog Input Event - 16-bit without time")?
        .push(gv(32, 3), "Analog Input Event - 32-bit with time")?
        .push(gv(32, 4), "Analog Input Event - 16-bit with time")?
        .push(
            gv(32, 5),
            "Analog Input Event - Single-precision floating point without time",
        )?
        .push(
            gv(32, 6),
            "Analog Input Event - Double-precision floating point without time",
        )?
        .push(
            gv(32, 7),
            "Analog Input Event - Single-precision floating point with time",
        )?
        .push(
            gv(32, 8),
            "Analog Input Event - Double-precision floating point with time",
        )?
        .push(gv(40, 0), "Analog Output Status - Default variation")?
        .push(gv(40, 1), "Analog Output Status - 32-bit with flags")?
        .push(gv(40, 2), "Analog Output Status - 16-bit with flags")?
        .push(
            gv(40, 3),
            "Analog Output Status - Single-precision floating point with flags",
        )?
        .push(
            gv(40, 4),
            "Analog Output Status - Double-precision floating point with flags",
        )?
        .push(gv(41, 0), "Analog Output - Default variation")?
        .push(gv(41, 1), "Analog Output - 32-bit")?
        .push(gv(41, 2), "Analog Output - 16-bit")?
        .push(gv(41, 3), "Analog Output - Single-precision floating point")?
        .push(gv(41, 4), "Analog Output - Double-precision floating point")?
        .push(gv(42, 0), "Analog Output Event - Default variation")?
        .push(gv(42, 1), "Analog Output Event - 32-bit without time")?
        .push(gv(42, 2), "Analog Output Event - 16-bit without time")?
        .push(gv(42, 3), "Analog Output Event - 32-bit with time")?
        .push(gv(42, 4), "Analog Output Event - 16-bit with time")?
        .push(
            gv(42, 5),
            "Analog Output Event - Single-precision floating point without time",
        )?
        .push(
            gv(42, 6),
            "Analog Output Event - Double-precision floating point without time",
        )?
        .push(
            gv(42, 7),
            "Analog Output Event - Single-preicions floating point with time",
        )?
        .push(
            gv(42, 8),
            "Analog Output Event - Double-preicions floating point with time",
        )?
        /*
        .push(
            gv(43, 1),
            "Analog Output Command Event - 32-bit without time",
        )?
        .push(
            gv(43, 2),
            "Analog Output Command Event - 16-bit without time",
        )?
        .push(
            gv(43, 3),
            "Analog Output Command Event - 32-bit with time",
        )?
        .push(
            gv(43, 4),
            "Analog Output Command Event - 16-bit with time",
        )?
        .push(
            gv(43, 5),
            "Analog Output Command Event - Single-precision floating point without time",
        )?
        .push(
            gv(43, 6),
            "Analog Output Command Event - Double-precision floating point without time",
        )?
        .push(
            gv(43, 7),
            "Analog Output Command Event - Single-precision floating point with time",
        )?
        .push(
            gv(43, 8),
            "Analog Output Command Event - Double-precision floating point with time",
        )?
         */
        .push(gv(50, 1), "Time and Date - Absolute time")?
        .push(
            gv(50, 3),
            "Time and Date - Absolute time at last recorded time",
        )?
        .push(
            gv(50, 4),
            "Time and Date - Indexed absolute time and long interval",
        )?
        .push(gv(51, 1), "Time and date CTO - Absolute time, synchronized")?
        .push(
            gv(51, 2),
            "Time and date CTO - Absolute time, unsynchronized",
        )?
        .push(gv(52, 1), "Time delay - Coarse")?
        .push(gv(52, 2), "Time delay - Fine")?
        .push(gv(60, 1), "Class objects - Class 0 data")?
        .push(gv(60, 2), "Class objects - Class 1 data")?
        .push(gv(60, 3), "Class objects - Class 2 data")?
        .push(gv(60, 4), "Class objects - Class 3 data")?
        .push(gv(80, 1), "Internal Indications - Packed format")?
        .push("group110", "Octet String")?
        .push("group111", "Octet String Event")?
        /*
        .push("group112", "Virtual Terminal Output Block")?
        .push("group113", "Virtual Terminal Event Data")?
         */
        .doc("Group/Variation")?
        .build()?;

    Ok(variation)
}
