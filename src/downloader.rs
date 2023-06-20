use csv::Writer;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Member {
    id: Uuid,
    name: String,
}

impl Member {
    pub fn members() -> Vec<Member> {
        let member_1 = Member {
            id: Uuid::new_v4(),
            name: "John".to_string(),
        };

        let member_2 = Member {
            id: Uuid::new_v4(),
            name: "Alice".to_string(),
        };

        let member_3 = Member {
            id: Uuid::new_v4(),
            name: "Bob".to_string(),
        };

        let member_4 = Member {
            id: Uuid::new_v4(),
            name: "Ade".to_string(),
        };

        vec![member_1, member_2, member_3, member_4]
    }
}

pub fn create_csv(members: &Vec<Member>, path: String) -> Result<(), anyhow::Error> {
    let mut wr = Writer::from_path(path).unwrap();

    for member in members {
        wr.serialize(member).unwrap();
    }

    wr.flush().unwrap();

    Ok(())
}


