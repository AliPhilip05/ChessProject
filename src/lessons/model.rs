//! Static contract shared by the browser Guidebook and terminal curriculum.

#[derive(Clone, Copy)]
pub struct Lesson {
    pub id: &'static str,
    pub track: &'static str,
    pub title: &'static str,
    pub concept: &'static str,
    pub prompt: &'static str,
    pub hint: &'static str,
    pub question: &'static str,
    pub choices: [&'static str; 3],
    pub answer: usize,
    pub solution_file: &'static str,
    pub reference_anchor: &'static str,
}

pub(super) const fn lesson(
    id: &'static str,
    track: &'static str,
    title: &'static str,
    concept: &'static str,
    prompt: &'static str,
    hint: &'static str,
    question: &'static str,
    choices: [&'static str; 3],
    answer: usize,
    solution_file: &'static str,
    reference_anchor: &'static str,
) -> Lesson {
    Lesson {
        id,
        track,
        title,
        concept,
        prompt,
        hint,
        question,
        choices,
        answer,
        solution_file,
        reference_anchor,
    }
}
