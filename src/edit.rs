use syn::visit_mut::{self, VisitMut};
use syn::{File, Item, ItemMod};

struct RemoveMacroRules;

impl VisitMut for RemoveMacroRules {
    fn visit_item_mod_mut(&mut self, i: &mut ItemMod) {
        if let Some((_, items)) = &mut i.content {
            remove_macro_rules_from_vec_item(items);
        }
        visit_mut::visit_item_mod_mut(self, i);
    }
}

pub fn remove_macro_rules(syntax_tree: &mut File) {
    remove_macro_rules_from_vec_item(&mut syntax_tree.items);
    RemoveMacroRules.visit_file_mut(syntax_tree);
}

fn remove_macro_rules_from_vec_item(items: &mut Vec<Item>) {
    items.retain(|item| match item {
        Item::Macro(_) => false,
        _ => true,
    });
}
