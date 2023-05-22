use crate::{Hanzi, generate_binary_numbers};

pub fn generate_combinations(hanzis: Vec<Hanzi>) -> Vec<Hanzi> {
    
    let mut result: Vec<Hanzi> = Vec::new();
    
    while result.len() != hanzis.len() {

        for hanzi in hanzis.iter() {
            
            if result.iter().any(|h| h.id == hanzi.id) {
                continue
            }

            match &hanzi.children {
                Some(children) => {
                    let all_children_has_result = children.iter()
                                                          .all(|c| result.iter()
                                                                         .any(|h| h.id == *c));
                                                                                        
                    if all_children_has_result == false {
                        continue 
                    }
                    
                    let children_result: Vec<&Hanzi> = children.iter()
                                                               .map(|child| {
                                                                   result.iter()
                                                                         .find(|h| h.id == *child)
                                                                         .expect("Cannot be None")
                                                               })
                                                               .collect();
                    
                    let binary_numbers = generate_binary_numbers(children_result.len() as u32)
                        .into_iter()
                        .filter(|binary_number| {
                            children_result.iter().enumerate().all(|(hanzi_index, hanzi)|
                                hanzi.children.is_some() || binary_number.chars().nth(hanzi_index).unwrap() == '0'
                            )
                        })
                        .collect::<Vec<String>>();

                    let mut hanzi_result: Vec<String> = Vec::new();

                    for binary_number in binary_numbers {
                        
                        let mut result_vec: Vec<String> = Vec::new();
                        
                        for (number_index, one_number) in binary_number.chars().enumerate() {
                            if result_vec.is_empty() {
                                match one_number {
                                    '0' => result_vec.push(children_result[number_index].id.clone()),
                                    '1' => {
                                        if let Some(res) = &children_result[number_index].result {
                                            result_vec.extend(res.iter().cloned());
                                        };
                                    },
                                    _ => ()
                                }
                            }
                            else {
                                match one_number {
                                    '0' => {
                                        for string in result_vec.iter_mut() {
                                            string.push_str(&children_result[number_index].id)
                                        }
                                    },
                                    '1' => {
                                        let mut new_result_vec: Vec<String> = Vec::new();
                                        for res in result_vec.iter() {
                                            if let Some(children) = &children_result[number_index].result {
                                                for child in children {
                                                    new_result_vec.push(format!("{}{}", res,child))
                                                }
                                            }
                                        }
                                        result_vec = new_result_vec;
                                    },
                                    _ => ()
                                }
                            }
                        }
                        hanzi_result.extend(result_vec);
                    }
                    
                    let mut h = hanzi.clone();
                    h.result = Some(hanzi_result);
                    result.push(h);
                },
                None => { 
                    let mut h = hanzi.clone();
                    h.result = Some(vec![h.id.clone()]);
                    result.push(h);
                }
            }
        }
    }
    
    result
}