pub fn boxes_from_items(items: f64, name: &String) -> f64 {
    let max_box_content: f64 = 1728.0;
    let multiplicator = get_stack_change(name.to_lowercase());
    let boxes: f64 = (items * multiplicator / max_box_content * 100.0).round() / 100.0;
    if boxes == 0.0 && items > 0.0 {
        return 0.01;
    } else {
        return boxes;
    }
}

pub fn item_from_boxes(boxes: f64, name: &String) -> f64 {
    let dividor = get_stack_change(name.to_ascii_lowercase());
    let box_content: f64 = 1728.0 / dividor;
    let items = boxes * box_content;
    if items.round() < items {
        return items.round() + 1.0;
    } else {
        return items.round();
    }
}

fn get_stack_change(checkname: String) -> f64 {
    let mut multiplicator = 1.0;
    if checkname.contains("chestplate") {
        multiplicator = 64.0;
    } else if checkname.contains("leggings") {
        multiplicator = 64.0;
    } else if checkname.contains("boots") {
        multiplicator = 64.0;
    } else if checkname.contains("helmet") {
        multiplicator = 64.0;
    } else if checkname.contains("sword") {
        multiplicator = 64.0;
    } else if checkname.contains("axe") {
        multiplicator = 64.0;
    } else if checkname.contains("shield") {
        multiplicator = 64.0;
    } else if checkname.contains("tunic") {
        multiplicator = 64.0;
    } else if checkname.contains("cap") {
        multiplicator = 64.0;
    } else if checkname.contains("pants") {
        multiplicator = 64.0;
    } else if checkname.contains("horse") {
        multiplicator = 64.0;
    } else if checkname.contains("brush") {
        multiplicator = 64.0;
    } else if checkname.contains("bundle") {
        multiplicator = 64.0;
    } else if checkname.contains("clock") {
        multiplicator = 4.0;
    } else if checkname.contains("recovery") {
        multiplicator = 1.0;
    } else if checkname.contains("fishing") {
        multiplicator = 64.0;
    } else if checkname.contains("steel") {
        multiplicator = 64.0;
    } else if checkname.contains("hoe") {
        multiplicator = 64.0;
    } else if checkname.contains("lead") {
        multiplicator = 64.0;
    } else if checkname.contains("compas") {
        multiplicator = 64.0;
    } else if checkname.contains("shear") {
        multiplicator = 64.0;
    } else if checkname.contains("shovel") {
        multiplicator = 64.0;
    } else if checkname.contains("spyglass") {
        multiplicator = 64.0;
    } else if checkname.contains("saddle") {
        multiplicator = 64.0;
    } else if checkname.contains("water") && checkname.contains("bucket") {
        multiplicator = 64.0;
    } else if checkname.contains("lava") {
        multiplicator = 64.0;
    } else if checkname.contains("bucket") {
        multiplicator = 4.0;
    } else if checkname.contains("potion") {
        multiplicator = 64.0;
    } else if checkname.contains("bow") {
        multiplicator = 64.0;
    } else if checkname.contains("totem") {
        multiplicator = 64.0;
    } else if checkname.contains("snowball") {
        multiplicator = 4.0;
    } else if checkname.contains("egg") {
        multiplicator = 4.0;
    } else if checkname.contains("sign") {
        multiplicator = 4.0;
    } else if checkname.contains("water") {
        multiplicator = 64.0;
    } else if checkname.contains("bottle") {
        multiplicator = 4.0;
    } else if checkname.contains("banner") {
        multiplicator = 4.0;
    } else if checkname.contains("ender") && checkname.contains("pear") {
        multiplicator = 4.0;
    } else if checkname.contains("stand") {
        multiplicator = 4.0;
    }
    multiplicator
}
