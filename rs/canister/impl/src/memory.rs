use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

const UPGRADES: MemoryId = MemoryId::new(0);
const EVENTS_V2_INDEX: MemoryId = MemoryId::new(1);
const EVENTS_V2_DATA: MemoryId = MemoryId::new(2);
const EVENTS_INDEX: MemoryId = MemoryId::new(3);
const EVENTS_DATA: MemoryId = MemoryId::new(4);
const STRING_TO_NUM_MAP: MemoryId = MemoryId::new(5);
const NUM_TO_STRING_INDEX: MemoryId = MemoryId::new(6);
const NUM_TO_STRING_DATA: MemoryId = MemoryId::new(7);
const STRING_TO_NUM_V2_MAP: MemoryId = MemoryId::new(8);
const NUM_TO_STRING_V2_INDEX: MemoryId = MemoryId::new(9);
const NUM_TO_STRING_V2_DATA: MemoryId = MemoryId::new(10);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: MemoryManager<DefaultMemoryImpl>
        = MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 128);
}

pub fn get_upgrades_memory() -> Memory {
    get_memory(UPGRADES)
}

pub fn get_events_index_memory() -> Memory {
    get_memory(EVENTS_INDEX)
}

pub fn get_events_data_memory() -> Memory {
    get_memory(EVENTS_DATA)
}

pub fn get_string_to_num_map_memory() -> Memory {
    get_memory(STRING_TO_NUM_MAP)
}

pub fn get_num_to_string_index_memory() -> Memory {
    get_memory(NUM_TO_STRING_INDEX)
}

pub fn get_num_to_string_data_memory() -> Memory {
    get_memory(NUM_TO_STRING_DATA)
}

pub fn get_events_v2_index_memory() -> Memory {
    get_memory(EVENTS_V2_INDEX)
}

pub fn get_events_v2_data_memory() -> Memory {
    get_memory(EVENTS_V2_DATA)
}

pub fn get_string_to_num_v2_map_memory() -> Memory {
    get_memory(STRING_TO_NUM_V2_MAP)
}

pub fn get_num_to_string_v2_index_memory() -> Memory {
    get_memory(NUM_TO_STRING_V2_INDEX)
}

pub fn get_num_to_string_v2_data_memory() -> Memory {
    get_memory(NUM_TO_STRING_V2_DATA)
}

fn get_memory(id: MemoryId) -> Memory {
    MEMORY_MANAGER.with(|m| m.get(id))
}
