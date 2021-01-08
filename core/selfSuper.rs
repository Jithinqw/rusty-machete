mod BBTCharCreate {

    pub enum Behavior {
        RUDE,
        RACIST,
        SARCASTIC,
        GEEK,
        KNOLEDGEALE,
        BEAUTIFUL,
        MUSCLE,
        INDIAN,
        AMERICAN,
    }

    pub enum Education {
        ACTRESS,
        MICROBIOLOGIST,
        THEORITICALPHYSIST,
        NEUROBIOLOGIST,
        EXPERIMENTALPHYSIST,
        ENGINEER,
        ASTROPHYSIST,
    }

    pub struct Character {
        name: &String,
        behavior: Vec<Behavior>,
        education: Education,
        age: i64,
    }

    impl Character {
        pub fn create_new_character(name:String, behavior:Vec<Behavior>, education: Education, age:i64) -> Character {
            Character {
                name:&name,
                behavior:
            }
        }
    }
}
