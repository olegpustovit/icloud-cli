//! Demonstrate the real Notes codec without creating a client or session.
use icloud_api::notes::{markdown, proto};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "# Example note\n\nMeasure before changing the **worker**.\n\n- [ ] Record a baseline\n- [x] Create a small fixture\n";
    let parsed = markdown::from_markdown(input)?;
    assert!(parsed.tables.is_empty());
    let encoded = proto::encode_note_body(&parsed.doc)?;
    let decoded = proto::decode_note_body(&encoded)?;
    let output = markdown::to_markdown(&decoded);
    for expected in [
        "Example note",
        "worker",
        "Record a baseline",
        "Create a small fixture",
    ] {
        assert!(
            output.contains(expected),
            "Missing fixture text: {expected}"
        );
    }
    println!("Offline Apple Notes codec example");
    println!("Markdown -> compressed protobuf -> Markdown\n");
    print!("{output}");
    println!("\nFixture text preserved. No client, login or network request.");
    Ok(())
}
