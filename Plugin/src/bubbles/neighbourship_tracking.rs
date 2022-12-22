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
    fn add(&mut self, a: u32, b: u32) {
        let key = (a,b);
        let key2 = (b,a);
        if let Some(neighbourship) = self.neighbourships.get_mut(&key) {
            neighbourship.in_contact_this_frame = true;
            neighbourship.contact_duration += 1;
        }  
        else{
            self.neighbourships.insert(key, BubbleNeigbourship {
                in_contact_this_frame: true,
                contact_duration: 1
            });
        }
        
        if let Some(neighbourship) = self.neighbourships.get_mut(&key2) {
            neighbourship.in_contact_this_frame = true;
            neighbourship.contact_duration += 1;
        } else {
            self.neighbourships.insert(key2, BubbleNeigbourship {
                in_contact_this_frame: true,
                contact_duration: 1
            });
        }
    }
    fn clear_in_contact_this_frame(&mut self) {
        for (_, neighbourship) in self.neighbourships.iter_mut() {
            neighbourship.in_contact_this_frame = false;
        }
    }
    
    fn clear(&mut self) {
        self.neighbourships.clear();
    }
}



struct BubbleNeigbourship {
    pub in_contact_this_frame: bool,
    pub contact_duration: u32
}





