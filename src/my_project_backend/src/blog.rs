use candid::CandidType;

#[derive(Clone, CandidType)]
pub struct Blog {
    title: String,
    date: u32, // 0 - 255
    content: String,
    tags: Vec<String>
}

impl Blog { // implementacja

    pub fn new(title: String, date: u32, content: String, tags: Vec<String>) -> Self {
        Self {
            title,
            date,
            content,
            tags,
        }
    }


}