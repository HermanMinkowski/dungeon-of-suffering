#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ItemKind {
    Sword,
    Key,
    Amulet,
    Map,
    Purse,
    Gold,
}

impl ItemKind {
    pub fn translation_key(&self) -> &'static str {
        match self {
            ItemKind::Sword => "sword",
            ItemKind::Key => "key",
            ItemKind::Amulet => "amulet",
            ItemKind::Map => "map",
            ItemKind::Purse => "pure",
            ItemKind::Gold => "gold",
        }
    }
}

#[derive(Debug)]
pub struct Item {
    pub kind: ItemKind,
    pub name: &'static str,
    pub amount: u32,
    pub max_amount: u32,
}

impl Item {
    pub fn new(kind: ItemKind, amount: u32, max_amount: u32) -> Self {
        Item {
            kind,
            name: kind.translation_key(),
            amount,
            max_amount,
        }
    }

    pub fn new_default(kind: ItemKind) -> Self {
        Item::new(kind, 0, u32::MAX)
    }
}
#[derive(Debug)]
pub struct Equipment {
    pub items: Vec<Item>,
}

impl Equipment {
    pub fn new(items: Vec<Item>) -> Self {
        Equipment { items }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn has(&self, name: &'static str) -> bool {
        for item in &self.items {
            if item.name == name {
                return true;
            }
        }

        false
    }

    pub fn init_equipment() -> Equipment {
        let sword = Item::new_default(ItemKind::Sword);
        let amulet = Item::new_default(ItemKind::Amulet);
        let map = Item::new_default(ItemKind::Map);
        let purse = Item::new_default(ItemKind::Purse);
        let gold = Item::new_default(ItemKind::Gold);

        Equipment::new(vec![sword, map, amulet, purse, gold])
    }
}
