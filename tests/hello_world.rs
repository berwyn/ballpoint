#[test]
fn hello_world() {
    let story = ballpoint::from_str(&include_str!("hello_world.ink")).expect("Unable to parse");
    let json = serde_json::to_string_pretty(&story).expect("Unable to serialize");

    pretty_assertions::assert_eq!(include_str!("hello_world.json"), json);
}
