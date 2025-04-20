// Adapted from the example code of the gettext-rs crate
// https://docs.rs/gettext-rs/latest/gettextrs/index.html

use formatx::formatx;
use gettextrs::*;

macro_rules! gettext_fmt {
    ($string:expr, $($args:expr),+ $(,)?) => {
        formatx!(gettext($string), $($args), +).unwrap()
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hellorust = "hellorust";
    let _ = TextDomain::new(hellorust)
        // .mo file will be placed in ./locale/de/LC_MESSAGES/hellorust.mo
        // IMPORTANT: Specify the directory containing the `locale` directory,
        // not the path to the `locale` directory.
        .prepend(".")
        // Ask gettext for UTF-8 strings. THIS CRATE CAN'T HANDLE NON-UTF-8 DATA!
        .codeset("UTF-8")
        .init();

    // `gettext()` simultaneously marks a string for translation and translates
    // it at runtime.
    println!("Translated: {}", gettext("Hello, world!"));

    // gettext supports plurals, i.e. you can have different messages depending
    // on the number of items the message mentions. This even works for
    // languages that have more than one plural form, like Russian or Czech.
    println!("Singular: {}", ngettext("One thing", "Multiple things", 1));
    println!("Plural: {}", ngettext("One thing", "Multiple things", 2));

    // gettext de-duplicates strings, i.e. the same string used multiple times
    // will have a single entry in the PO and MO files. However, the same words
    // might have different meaning depending on the context. To distinguish
    // between different contexts, gettext accepts an additional string:
    println!(
        "With context: {}",
        pgettext("This is the context", "Hello, world!")
    );
    println!(
        "Plural with context: {}",
        npgettext("This is the context", "One thing", "Multiple things", 2)
    );

    let formatx_str = formatx!(gettext("Here is a number: {}"), 42).unwrap();
    println!("{formatx_str}");

    let order_switch = formatx!(gettext("zero: {}, one: {}"), 0, 1).unwrap();
    println!("{order_switch}");

    // This does not make it into the po files.
    let macro_text = gettext_fmt!("format through macro {}", 123);
    println!("{macro_text}");

    Ok(())
}
