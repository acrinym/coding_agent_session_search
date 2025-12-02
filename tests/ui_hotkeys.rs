use coding_agent_search::ui::tui::footer_legend;

#[test]
fn footer_mentions_editor_and_clear_keys() {
    // Simplified footer shows essential keys only
    let short = footer_legend(false);
    assert!(
        short.contains("Enter view"),
        "short footer should show Enter view"
    );
    assert!(
        short.contains("Esc quit"),
        "short footer should show Esc quit"
    );
    assert!(
        short.contains("F1 help"),
        "short footer should show F1 help"
    );
}

#[test]
fn help_includes_detail_find_hotkeys() {
    let lines = coding_agent_search::ui::tui::help_lines(
        coding_agent_search::ui::components::theme::ThemePalette::dark(),
    );
    let text: String = lines.iter().map(|l| l.to_string()).collect();
    assert!(
        text.contains("/ detail-find"),
        "help should mention detail-find shortcut"
    );
    assert!(
        text.contains("n/N"),
        "help should mention n/N navigation in detail-find"
    );
}
