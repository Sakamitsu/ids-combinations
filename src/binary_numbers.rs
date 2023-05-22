pub fn generate_binary_numbers(n: u32) -> Vec<String> {
    
    let mut result = Vec::new();
    for i in 0..(2u32.pow(n)) {
        let binary = format!("{:0width$b}", i, width = n as usize);
        result.push(binary);
    }
    result
}