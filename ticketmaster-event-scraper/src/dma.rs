use std::collections::HashMap;

pub fn get_hashmap() -> HashMap<&'static str, &'static str> {
    let mut city_to_dma = HashMap::new();
    city_to_dma.insert("Atlanta", "220");
    
    return city_to_dma;
}