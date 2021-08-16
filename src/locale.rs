use cursive::{
    align::HAlign,
    traits::Scrollable,
    view::{Boxable, Identifiable},
    views::{Dialog, EditView, LinearLayout, SelectView, TextView},
    Cursive,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref LANG: &'static str = include_str!("../assets/locale.txt");
}

pub fn locale_menu() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(
                    EditView::new()
                        // update results every time the query changes
                        .on_edit(on_edit)
                        // submit the focused (first) item of the matches
                        .on_submit(on_submit)
                        .with_name("query"),
                )
                .child(
                    SelectView::new()
                        .with_all_str(LANG.lines())
                        .on_submit(show_next_window)
                        .h_align(HAlign::Center)
                        .with_name("matches")
                        .scrollable(),
                )
                .fixed_height(20),
        )
        .button("Quit", Cursive::quit)
        .title("Please select $LANG"),
    );

    siv.run();
}

fn on_edit(siv: &mut Cursive, query: &str, _cursor: usize) {
    let matches = search_fn(LANG.lines(), query);
    siv.call_on_name("matches", |v: &mut SelectView| {
        v.clear();
        v.add_all_str(matches);
    });
}

fn search_fn<'a, 'b, T: std::iter::IntoIterator<Item = &'a str>>(
    items: T,
    query: &'b str,
) -> Vec<&'a str> {
    items
        .into_iter()
        .filter(|&item| {
            let item = item.to_lowercase();
            let query = query.to_lowercase();
            item.contains(&query)
        })
        .collect()
}

fn on_submit(siv: &mut Cursive, query: &str) {
    let matches = siv.find_name::<SelectView>("matches").unwrap();
    if matches.is_empty() {
        show_next_window(siv, query);
    } else {
        let city = &*matches.selection().unwrap();
        show_next_window(siv, city);
    };
}

fn show_next_window(siv: &mut Cursive, city: &str) {
    siv.pop_layer();
    let text = format!("Your locale is {}", city);
    siv.add_layer(Dialog::around(TextView::new(text)).button("Ok", |s| s.quit()));
}
