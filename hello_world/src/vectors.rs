struct Box {
    width: i32,
}

pub fn get_boxes() {
    let boxes = vec![ 
        Box{ width: 10 },
        Box{ width: 20 },
        Box{ width: 30 }
    ];

    println!("Boxes leng: {:?}", boxes.len());
    
    for box_ in boxes {
        println!("Width: {}", box_.width);
    }
    
    // Wouldn't work because boxes is moved to the for loop
    // we would need to use a reference(borrow)
    // println!("Boxes leng: {:?}", boxes.len());
}