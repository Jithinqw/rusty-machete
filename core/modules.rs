mod MySystem {

    pub enum MemoryUnit {
        GB,
        MB,
        TB
    }

    #[derive(Debug)]
    pub struct System {
        SystemName: &String,
        SystemMemory: i64,
        SystemMemoryUnit: MemoryUnit,
        SystemHardSpace: i64,
        SystemHardSpaceUnit: MemoryUnit,
    }

    impl System {

        fn create_new_system(name:&String, memory:i64, unit:MemoryUnit, hard_space:i64, hard_space_unit: MemoryUnit) -> System {
            System {
                SystemName: name,
                SystemMemory: memory,
                SystemMemoryUnit: unit,
                SystemHardSpace:hard_space,
                SystemHardSpaceUnit: hard_space_unit
            }
        }
    }
}

fn main() {
    let modw = MySystem::System::create_new_system(&String::from("Jithin"), 232, MySystem::MemoryUnit::GB, 233, MySystem::MemoryUnit::TB);
    println!("{:?}", modw)
}
