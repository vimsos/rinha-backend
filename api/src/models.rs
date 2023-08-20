use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use uuid::Uuid;

#[derive(Deserialize)]
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

pub struct PersonEntity {
    pub id: Uuid,
    pub handle: CompactString,
    pub payload: String,
    pub search: String,
}

#[derive(Serialize)]
pub struct PersonPayload<'a> {
    pub id: &'a Uuid,
    #[serde(rename = "nome")]
    pub name: &'a CompactString,
    #[serde(rename = "apelido")]
    pub handle: &'a CompactString,
    #[serde(rename = "nascimento")]
    pub birth: &'a CompactString,
    #[serde(rename = "stack")]
    pub stacks: &'a Option<SmallVec<[CompactString; 10]>>,
}

impl TryFrom<PersonPostDTO> for PersonEntity {
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
        let payload = PersonPayload {
            id: &id,
            name: &value.name,
            handle: &value.handle,
            birth: &value.birth,
            stacks: &value.stacks,
        };

        let mut search_buf = String::with_capacity(400);
        search_buf.push_str(&value.handle);
        search_buf.push(' ');
        search_buf.push_str(&value.name);
        search_buf.push(' ');
        if let Some(stacks) = &value.stacks {
            stacks.iter().for_each(|s| {
                search_buf.push_str(s);
                search_buf.push(' ');
            })
        }

        Ok(Self {
            id,
            search: search_buf,
            payload: serde_json::to_string(&payload).unwrap(),
            handle: value.handle,
        })
    }
}
