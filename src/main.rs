use tmoe::{fill_out_the_form, locale};

fn main() {
    // When the user starts for the first time, the locale ($LANG) needs to be selected.
    locale::locale_menu();
    fill_out_the_form();
}
