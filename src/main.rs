use std::env;
use rand::distributions::Distribution;

fn update_weights(weights: &mut Vec<f32>, width: u32, sigma: f32, index: usize, sign: f32) {
    let x = ((index as u32) % width) as i32;
    let y = ((index as u32) / width) as i32;
    
    for iy in 0..width {
        let mut dist_y = ((iy as i32) - y).abs() as u32;
        dist_y = dist_y.min(width - dist_y);

        for ix in 0..width {
            let mut dist_x = ((ix as i32) - x).abs() as u32;
            dist_x = dist_x.min(width - dist_x);
            
            let distance_squared = (dist_x * dist_x + dist_y * dist_y) as f32;
            let weight = (-distance_squared / (2.0 * sigma * sigma)).exp();
            weights[(ix + iy * width) as usize] += weight * sign;
        }
    }
}

fn find_tightest_cluster(binary_pattern: &Vec<u8>, weights: &Vec<f32>, width: u32, minority_pixel: u8) -> usize {
    let mut highest_weight : f32 = -f32::MAX;
    let mut highest_index : usize = usize::MAX;
    for iy in 0..width {
        for ix in 0..width {
            let index = (ix + iy * width) as usize;
            if binary_pattern[index] == minority_pixel {
                let weight = weights[index];
                if highest_weight < weight {
                    highest_weight = weight;
                    highest_index = index;
                }
            }
        }
    }
    highest_index
}
fn find_largest_void(binary_pattern: &Vec<u8>, weights: &Vec<f32>, width: u32, minority_pixel: u8) -> usize {
    let mut lowest_weight : f32 = f32::MAX;
    let mut lowest_index : usize = usize::MAX;
    for iy in 0..width {
        for ix in 0..width {
            let index = (ix + iy * width) as usize;
            if binary_pattern[index] == (1 - minority_pixel) {
                let weight = weights[index];
                if lowest_weight > weight {
                    lowest_weight = weight;
                    lowest_index = index;
                }
            }
        }
    }
    lowest_index
}

fn main() {
    // Read command line arguments
    let mut args = env::args()
        .skip(1); // Skip program name
    let filename = args.next().expect("Expected filename");
    let width = args.next().expect("Expected width").parse::<u32>().expect("Width is not a valid number");

    // Read optional command line arguments
    let mut sigma = 1.5;
    if let Some(value) = args.next() {
        sigma = value.parse::<f32>().expect("Sigma is not a valid number");
    }

    let num_pixels = (width * width) as usize;

    let mut rng = rand::thread_rng();
    let distribution = rand::distributions::Uniform::from(0..num_pixels);

    // Create initial binary pattern
    let mut initial_binary_pattern : Vec<u8> = vec![0; num_pixels];
    let mut initial_weights : Vec<f32> = vec![0.0; num_pixels];

    let num_initial_ones = (num_pixels * 26) / (16 * 16);
    for _ in 0..num_initial_ones {
        let index = distribution.sample(&mut rng);
        initial_binary_pattern[index] = 1;
        update_weights(&mut initial_weights, width, sigma, index, 1.0);
    }

    loop {
        // Remove '1' in the tighest cluster (most concentrated ones)
        let tightest_cluster = find_tightest_cluster(&initial_binary_pattern, &initial_weights, width, 1);
        initial_binary_pattern[tightest_cluster] = 0;
        update_weights(&mut initial_weights, width, sigma, tightest_cluster, -1.0);

        // Add '1' in the largest void (least concentrated ones)
        let largest_void = find_largest_void(&initial_binary_pattern, &initial_weights, width, 1);
        initial_binary_pattern[largest_void] = 1;
        update_weights(&mut initial_weights, width, sigma, largest_void, 1.0);

        if tightest_cluster == largest_void {
            break;
        }
    }

    // Count the number of ones in the initial binary pattern
    let mut num_ones : u32 = 0;
    for i in 0..num_pixels {
        num_ones += initial_binary_pattern[i] as u32;
    }

    let mut ranks : Vec<u32> = vec![0; num_pixels];

    // Phase 1
    let mut binary_pattern = initial_binary_pattern.clone();
    let mut weights = initial_weights.clone();
    let mut rank = num_ones;
    while rank > 0 {
        // Remove '1' in the tighest cluster (most concentrated ones)
        let tightest_cluster = find_tightest_cluster(&binary_pattern, &weights, width, 1);
        binary_pattern[tightest_cluster] = 0;
        update_weights(&mut weights, width, sigma, tightest_cluster, -1.0);

        rank -= 1;
        ranks[tightest_cluster] = rank;
    }

    // Phase 2
    let mut binary_pattern = initial_binary_pattern.clone();
    let mut weights = initial_weights.clone();
    let mut rank = num_ones;
    while rank < ((width * width) / 2) {
        // Add '1' in the largest void (least concentrated ones)
        let largest_void = find_largest_void(&binary_pattern, &weights, width, 1);
        binary_pattern[largest_void] = 1;
        update_weights(&mut weights, width, sigma, largest_void, 1.0);

        ranks[largest_void] = rank;
        rank += 1;
    }

    // Reverse meaning of minority pixel from '1' to '0' by reversing the weights
    let mut weights = vec![0.0; num_pixels];
    for i in 0..num_pixels {
        if binary_pattern[i] == 0 {
            update_weights(&mut weights, width, sigma, i, 1.0);
        }
    }

    // Phase 3
    while rank < (width * width) {
        // Add '1' in the tightest cluster (most concentrated zeroes)
        let tightest_cluster = find_tightest_cluster(&binary_pattern, &weights, width, 0);
        binary_pattern[tightest_cluster] = 1;
        update_weights(&mut weights, width, sigma, tightest_cluster, -1.0);

        ranks[tightest_cluster] = rank;
        rank += 1;
    }

    // Store results to image
    let mut image_buffer : Vec<u8> = vec![0; num_pixels * 3];
    for i in 0..num_pixels {
        let value = ((ranks[i] * 256) / (num_pixels as u32)) as u8;
        image_buffer[i * 3 + 0] = value;
        image_buffer[i * 3 + 1] = value;
        image_buffer[i * 3 + 2] = value;
    }
    image::save_buffer(filename, &image_buffer, width, width, image::ColorType::Rgb8).unwrap();
}
