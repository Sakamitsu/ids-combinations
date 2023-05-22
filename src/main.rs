mod hanzi;
mod deserialize;
mod binary_numbers;
mod combinations;
mod serialize;

use hanzi::Hanzi;
use deserialize::deserialize_from_file;
use binary_numbers::generate_binary_numbers;
use combinations::generate_combinations;
use serialize::serialize_and_write_to_file;

#[cfg(test)]
mod tests;

fn main() {

    let hanzis = deserialize_from_file("src/files/input.json");
    let result: Vec<Hanzi> = generate_combinations(hanzis);
    serialize_and_write_to_file(result);

}
