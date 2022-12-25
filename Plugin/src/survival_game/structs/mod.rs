pub enum ResourceType{
    Wood,
    Stone,
    Metal,
    Coal,
}
pub enum BuildingType{
    Chest,
    CraftingStation,
    Nexus,
    Wall,
    Turret,
    MinionSpawner
}


pub enum ItemType {
    Resource(ResourceType),
    Building(BuildingType),
}


// every item needs to belong to a container
pub struct ItemInstance {
    pub item_type: ItemType,
    pub quantity: u32,
}
pub struct ItemContainer {
    pub items: Vec<ItemInstance>,
}