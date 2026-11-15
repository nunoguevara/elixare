/* You can add or append pub(super) mod your_boilerplate_name; so the register and the progam
   recognize it (NECESSARY) */
pub(super) mod example;

use std::collections::HashMap;

type Generator = fn() -> &'static str;

pub fn registry() -> HashMap<&'static str, Generator> {
    HashMap::from([
        /* You can add or append your boilerplate to the registry with the following format
           "--example" for the option or argument to invoke it
           "example::generate" for the invocation of the boilerplate template you've provided, 
           you may change the "exanple" to match your file name that defines the boilerplate 
           (NECESSARY) */
        ("--example", example::generate as Generator), 
    ])
}
