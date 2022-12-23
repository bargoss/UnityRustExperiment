use std::collections::HashMap;



// efficient neighbourship tracking where each node can have multiple neighbours
struct BubbleNeigbourships {
    neighbourships: HashMap<(u32,u32), BubbleNeigbourship>,
}
impl BubbleNeigbourships {
    fn new() -> BubbleNeigbourships {
        BubbleNeigbourships {
            neighbourships: HashMap::new(),
        }
    }
    fn record_collision(&mut self, a: u32, b: u32) {
        // swap a b if a > b
        let (a,b) = if a > b { (b,a) } else { (a,b) };
        let key = (a,b);
        if let Some(neighbourship) = self.neighbourships.get_mut(&key) {
            neighbourship.colliding = true;
            neighbourship.contact_duration += 1;
        }  
        else{
            self.neighbourships.insert(key, BubbleNeigbourship {
                colliding: true,
                contact_duration: 1
            });
        }
    }
    fn reset_colliding(&mut self) {
        for (_, neighbourship) in self.neighbourships.iter_mut() {
            neighbourship.colliding = false;
        }
    }
    fn on_update(&mut self) {
        for (_, neighbourship) in self.neighbourships.iter_mut() {
            neighbourship.colliding = false;
        }
    }
    
    fn clear(&mut self) {
        self.neighbourships.clear();
    }
}



struct BubbleNeigbourship {
    pub colliding: bool,
    pub contact_duration: u32
}





