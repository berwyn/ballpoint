#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    /// A line of dialogue that's "glued" to the previous one.
    /// # Example
    /// ```ink
    /// === hurry_home ===
    /// We hurried home <>
    /// -> to_saville_row
    ///
    /// === to_saville_row ===
    /// to Saville Row
    /// -> as_fast_as_we_could
    ///
    /// === as_fast_as_we_could ===
    /// <> as fast as we could.
    /// ```
    /// Will produce the text
    /// ```txt
    /// We hurried home to Saville Row as fast as we could.
    /// ```
    Glue(String),

    /// Text that can be read by the engine, but won't be shown as part of dialogue.
    /// # Example
    /// ```ink
    /// A line of normal text. # colour it blue
    /// ```
    Tag(String),

    /// Player input comes in the form of text choices.
    /// # Example
    /// ```ink
    /// Hello, world!
    /// * Hello back!
    ///   Nice to hear from you.
    /// ```
    /// Will produce
    /// ```txt
    /// Hello, world!
    /// 1: Hello back!
    ///
    /// > 1
    /// Hello back!
    /// Nice to hear from you.
    /// ```
    Choice(String),
    Comment(String),
    Suppression(String),

    /// Moves the story from one knot to another.
    /// # Example
    /// ```ink
    /// === back_in_london ===
    /// We arrived at 9.45pm exactly.
    /// -> hurry_home
    ///
    /// === hurry_home ===
    /// We hurried home to Savile Row as fast as we could.
    /// ```
    Divert(String),
}
