#[path = "../src/server_rendering_hard.rs"]
mod exercise;

#[test]
fn renderer_escapes_untrusted_text_and_keeps_its_wrapper() {
    assert_eq!(
        exercise::render_notice("<alert>", "A & B > C \"quoted\" 'text'"),
        "<aside><strong>&lt;alert&gt;</strong><p>A &amp; B &gt; C &quot;quoted&quot; &#39;text&#39;</p></aside>"
    );
}
