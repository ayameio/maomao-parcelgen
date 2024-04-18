use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use image::{Rgb, RgbImage}; 


fn main() {
    // Initialize a 50x50 array filled with -1
    let mut array_2d: Vec<Vec<i32>> = vec![vec![-1; 50]; 50];
    // Update the array based on random rules
    construct(&mut array_2d);

    // Replace negatives with zero
    zero_out_negatives(&mut array_2d);

    // Print the content of the array with better formatting
    
    
    // Create an image based on the array
    let image: image::ImageBuffer<Rgb<u8>, Vec<u8>> = create_image(&array_2d);
    
    // Save the image to a file
    image.save("output.png").expect("Failed to save image");
    print_array(&array_2d);

}

fn print_array(array: &Vec<Vec<i32>>) {
    for row in array {
        for element in row {
            print!("{:<4}", element);
        }
        println!();
    }
    for _ in 0..50 {
        print!("---- ");
    }
    println!();
}

fn can_fit(array: &Vec<Vec<i32>>, x: usize, y: usize, rule: usize) -> bool {
    for i in 0..rule {
        for j in 0..rule {
            let new_x: usize = x + i;
            let new_y: usize = y + j;

            if new_x >= 50 || new_y >= 50 || array[new_x][new_y] != -1 {
                return false;
            }
        }
    }
    true
}

fn construct(array: &mut Vec<Vec<i32>>) {
    let rules: Vec<(usize, i32)> = vec![(0, 70), (1, 10), (3, 25), (5, 25), (7, 25)];
    let weighted_index: WeightedIndex<i32> = WeightedIndex::new(rules.iter().map(|&(_, weight)| weight)).unwrap();
    let mut rng: rand::prelude::ThreadRng = thread_rng();

    for x in 0..50 {
        for y in 0..50 {
            if array[x][y] != -1 {
                continue;
            }

            // Choose a rule based on probabilities
            let rule_index: usize = weighted_index.sample(&mut rng);
            let rule: usize = rules[rule_index].0;

            // If the diagonal is not empty, choose the smallest rule number
            if !can_fit(array, x, y, rule) {
                let min_rule: usize = rules.iter().min_by_key(|&&(num, _)| num).unwrap().0;
                array[x][y] = min_rule as i32;
            } else {
                // Fill the square of diagonal size rule with -rule
                for i in 0..rule {
                    for j in 0..rule {
                        let new_x: usize = x + i;
                        let new_y: usize = y + j;
                        if new_x < 50 && new_y < 50 {
                            array[new_x][new_y] = 0;
                        }
                    }
                }

                // Place the current rule number
                array[x][y] = rule as i32;
            }
        }
    }
}

fn zero_out_negatives(array: &mut Vec<Vec<i32>>) {
    for row in array.iter_mut() {
        for element in row.iter_mut() {
            if *element < 0 {
                *element = 0;
            }
        }
    }
}

fn create_image(array: &Vec<Vec<i32>>) -> RgbImage {
    let mut image: image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(50, 50);

    for x in 0..50 {
        for y in 0..50 {
            let pixel_color: Rgb<u8> = match array[x][y] {
                0 => Rgb([255, 255, 255]), // White
                1 => Rgb([0, 0, 255]),     // Blue
                3 => Rgb([255, 0, 0]),     // Red
                5 => Rgb([0, 255, 0]),     // Green
                7 => Rgb([0, 0, 0]),       // Black
                _ => panic!("Invalid value in the array"),
            };

            image.put_pixel(x as u32, y as u32, pixel_color);
        }
    }

    image
}