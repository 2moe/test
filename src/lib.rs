// Some code from cursive's demo project

use cursive::{
    traits::*,
    views::{Dialog, EditView, LinearLayout, ListView, SelectView, TextArea, TextView},
};

pub mod locale;

pub fn fill_out_the_form() {
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::new()
            .title("Configure a new container")
            .button("Ok", |s| s.quit())
            .content(
                ListView::new()
                    .child(
                        "Distro",
                        SelectView::new()
                            .popup()
                            .item_str("arch")
                            .item_str("alpine")
                            .item_str("debian")
                            .item_str("fedora")
                            .item_str("kali")
                            .item_str("ubuntu"),
                    )
                    .child("Name", EditView::new().fixed_width(20))
                    .child("Description(optional)", TextArea::new().min_height(3))
                    .child("Timezone", EditView::new().fixed_width(20))
                    // .child(
                    //     "Mount SD?",
                    //     Checkbox::new().on_change(|s, checked| {
                    //         for name in &["sd_dir1", "sd_dir2"] {
                    //             s.call_on_name(name, |view: &mut EditView| {
                    //                 view.set_enabled(checked)
                    //             });
                    //             if checked {
                    //                 s.focus_name("sd_dir1").unwrap();
                    //             }
                    //         }
                    //     }),
                    // )
                    .child(
                        "sd dir",
                        SelectView::new()
                            .popup()
                            .item_str("/storage/self/primary")
                            .item_str("/sdcard")
                            .item_str("/storage/emulated/0")
                            .item_str("/media/sd"),
                    )
                    .delimiter()
                    .child(
                        "Example",
                        LinearLayout::horizontal().child(TextView::new("mount_source:mount_point")),
                    )
                    .with(|list| {
                        for i in 0..=20 {
                            list.add_child(&format!("mount arg {}", i), EditView::new());
                        }
                    })
                    .scrollable(),
            ),
    );
    siv.run();
}
