use compact_str::CompactString;
use serde::Deserialize;
use smallvec::SmallVec;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct PersonPostDTO {
    #[serde(rename = "nome")]
    pub name: CompactString,
    #[serde(rename = "apelido")]
    pub handle: CompactString,
    #[serde(rename = "nascimento")]
    pub birth: CompactString,
    #[serde(rename = "stack")]
    pub stacks: Option<SmallVec<[CompactString; 10]>>,
}

#[derive(Deserialize, Debug)]
pub struct Person {
    pub id: Uuid,
    pub handle: String,
    pub payload: String,
    pub search_vector: Option<String>,
}

impl TryFrom<PersonPostDTO> for Person {
    type Error = ();

    fn try_from(value: PersonPostDTO) -> Result<Self, ()> {
        if value.handle.len() > 32 {
            return Err(());
        }
        if value.name.len() > 100 {
            return Err(());
        }
        if value.birth.len() != 10 {
            return Err(());
        }
        for (split, length) in value.birth.split('-').zip([4usize, 2, 2]) {
            if split.len() != length {
                return Err(());
            }
            if split.parse::<u32>().ok().is_none() {
                return Err(());
            }
        }

        let id = Uuid::now_v7();
        Ok(Self {
            id,
            handle: value.handle.to_string(),
            payload: match value.stacks {
                Some(stack) => {
                    let mut stack_buf = String::with_capacity(200);
                    stack
                        .iter()
                        .for_each(|s| stack_buf.push_str(&format!(r#""{s}","#)));
                    stack_buf.pop();
                    format!(
                        r#"{{"id":"{}","apelido":"{}","nascimento":"{}","stack":[{}]}}"#,
                        id,
                        value.handle.as_str(),
                        value.birth,
                        stack_buf
                    )
                }
                None => format!(
                    r#"{{"id":"{}","apelido":"{}","nascimento":"{}","stack":null}}"#,
                    id,
                    value.handle.as_str(),
                    value.birth
                ),
            },
            search_vector: Some("todo!()".to_owned()),
        })
    }
}
