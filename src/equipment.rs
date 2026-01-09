use rust_i18n::t;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ItemKind {
    Bread,
    Notice,
    Sword,
    Key,
    Amulet,
    Map,
    Purse,
    Gold,
    Coal
}

impl ItemKind {
    pub fn translation_key(&self) -> &'static str {
        match self {
            ItemKind::Sword => "object.sword",
            ItemKind::Bread => "object.bread",
            ItemKind::Notice => "object.notice",
            ItemKind::Key => "object.key",
            ItemKind::Amulet => "object.amulet",
            ItemKind::Map => "object.map",
            ItemKind::Purse => "object.purse",
            ItemKind::Gold => "object.gold",
            ItemKind::Coal => "object.coal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Item {
    pub kind: ItemKind,
    pub amount: u32,
    pub max_amount: u32,
}

impl Item {
    pub fn new(kind: ItemKind, amount: u32, max_amount: u32) -> Self {
        Item {
            kind,
            amount,
            max_amount,
        }
    }

    pub fn new_default(kind: ItemKind) -> Self {
        Item::new(kind, 1, u32::MAX)
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
        if let Some(existing) = self.items.iter_mut().find(|i| i.kind == item.kind) {
            existing.amount += item.amount;
        } else {
            self.items.push(item);
        }
    }

    pub fn has(&self, item_kind: ItemKind) -> bool {
        self.items.iter().any(|item| item.kind == item_kind)
    }

    pub fn remove(&mut self, item_kind: ItemKind) {
        if let Some(existing) = self.items.iter_mut().find(|i| i.kind == item_kind) {
            if existing.amount > 1 {
                existing.amount -= 1;
            } else {
                self.items.retain(|i| i.kind != item_kind);
            }
        }
    }

    pub fn init_equipment() -> Equipment {
        let bread = Item::new_default(ItemKind::Bread);

        Equipment::new(vec![bread])
    }

    pub fn list(&self) -> String {
        let mut list = String::new();

        list.push_str(&format!("{}\n", t!("bag.content")));

        for item in &self.items {
            let amount = if item.amount > 1 {
                format!("{} ", item.amount)
            } else {
                "".to_string()
            };

            list.push_str(&format!(
                "* {}{}\n",
                amount,
                t!(item.kind.translation_key())
            ));
        }

        list
    }
}
