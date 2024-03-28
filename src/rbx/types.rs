/// Represents the UniverseId of a Roblox experience.
#[derive(Debug, Clone, Copy)]
pub struct UniverseId(pub u64);

/// Represents the PlaceId of a specific place within a Roblox experience.
#[derive(Debug, Clone, Copy)]
pub struct PlaceId(pub u64);

// Number of items to return.
#[derive(Debug, Clone, Copy)]
pub struct ReturnLimit(pub u64);

/// Represents a Roblox user's ID.
#[derive(Debug, Clone, Copy)]
pub struct RobloxUserId(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct PageSize(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct GroupId(pub u64);
