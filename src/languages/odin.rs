pub(super) fn generate() -> &'static str {
r#"packages main

import "core:fmt"

main :: proc() {
    fmt.println("Hello world!")
}
"#
}
