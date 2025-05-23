use super::*;

#[test]
fn test_form_render_block() {
    let form = test_form();
    let mut got = Buffer::empty(test_big_rect());

    form.render_block(got.area, &mut got);

    let want = Buffer::with_lines(vec![
        "                                                        ",
        "    Add New Item                                        ",
        "   ╭────────────────────────────────────────────────╮   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   ╰────────────────────────────────────────────────╯   ",
        "                                                        ",
    ]);

    assert_eq!(want, got)
}
