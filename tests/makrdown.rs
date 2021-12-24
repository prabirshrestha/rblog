use rblog::markdown::markdown_to_html;

#[test]
fn should_render_header_with_id() {
    let input = "# Heading 1 {#heading1}";
    let output = markdown_to_html(input);
    assert_eq!(output, "<h1 id=\"heading1\">Heading 1</h1>\n");
}
