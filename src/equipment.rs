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
            ItemKind::Purse => "object.pure",
            ItemKind::Gold => "object.gold",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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

    pub fn has(&self, item_kind: ItemKind) -> bool {
        self.items.iter().position(|item| item.kind == item_kind).is_some()
    }

    pub fn remove(&mut self, item_kind: ItemKind) {
        if let Some(position) = self.items.iter().position(|item| item.kind == item_kind) {
            self.items.remove(position);
        }
    }

    pub fn init_equipment() -> Equipment {
        let bread = Item::new_default(ItemKind::Bread);

        Equipment::new(vec![bread])
    }

    pub fn list(&self) {
        println!("{}", t!("bag.content"));

        self.items.clone().into_iter().for_each(|item| {
            print!("* ");

            if item.amount > 1 {
                print!("{}", item.amount);
            }

            println!("{}", t!(item.kind.translation_key()));
        });
    }
}
