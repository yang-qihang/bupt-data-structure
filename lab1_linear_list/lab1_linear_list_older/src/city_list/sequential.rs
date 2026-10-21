use crate::city_list::sequential::ListError::{DuplicateCityName, IndexOutOfBounds};
use std::io::{self, Write};

// Invariant:
// - city names must be unique;
// - indexing is zero-based;
// - insert_at(len, city) is allowed and appends;
// - insert_at(index > len, city) is out of bounds;
// - remove_at(index >= len) returns None;
// - distance search returns cities in their current list order.
#[derive(Debug, PartialEq, Clone)]
struct City {
    name: String,
    point: Point,
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Clone)]
enum ListError {
    DuplicateCityName,
    IndexOutOfBounds,
}

impl City {
    fn within_distance_of(&self, compare: &Point, distance: f64) -> bool {
        (self.point.x - compare.x).powi(2) + (self.point.y - compare.y).powi(2) <= distance.powi(2)
    }
}

struct CityList {
    city_data: Vec<City>,
}
impl CityList {
    // 0. creation
    fn new() -> Self {
        CityList {
            city_data: Vec::new(),
        }
    }

    // 1. insertion
    // 1.0 push at the front
    fn push_front(&mut self, city: City) -> Result<(), ListError> {
        if self.city_data.iter().any(|person| person.name == city.name) {
            return Err(DuplicateCityName);
        }
        self.city_data.insert(0, city);
        Ok(())
    }

    // 1.1 insert at the i-th position
    // the new item becomes the i-th element
    fn insert_at(&mut self, index: usize, city: City) -> Result<(), ListError> {
        if self.city_data.iter().any(|person| person.name == city.name) {
            return Err(DuplicateCityName);
        }
        if self.city_data.len() < index {
            return Err(IndexOutOfBounds);
        }
        self.city_data.insert(index, city);
        Ok(())
    }

    // 2. deletion
    // 2.0 pop at the front
    fn pop_front(&mut self) -> Option<City> {
        if self.city_data.is_empty() {
            return None;
        }
        Some(self.city_data.remove(0))
    }

    // 2.1 remove at the i-th position
    fn remove_at(&mut self, index: usize) -> Option<City> {
        if self.city_data.len() <= index {
            return None;
        }
        Some(self.city_data.remove(index))
    }

    // 3. modification
    // 3.0 update at the i-th position
    fn update_at(&mut self, index: usize, city: City) -> Result<(), ListError> {
        if self.city_data.len() <= index {
            return Err(IndexOutOfBounds);
        }
        let before_name = &self.city_data.get(index).unwrap().name;
        if city.name != *before_name && self.city_data.iter().any(|elem| elem.name == city.name) {
            return Err(DuplicateCityName);
        }
        let tmp_handle = self.city_data.get_mut(index).unwrap();
        *tmp_handle = city;
        Ok(())
    }

    // 4. search
    // 4.0 find by name
    fn find_by_name(&self, name: &str) -> Option<&City> {
        self.city_data.iter().find(|elem| elem.name == name)
    }

    // 4.1 find cities that are within a give distance from some point
    fn cities_within_distance_of(&self, point: &Point, max_distance: f64) -> Vec<&City> {
        self.city_data
            .iter()
            .filter(|elem| elem.within_distance_of(point, max_distance))
            .collect()
    }
}

pub fn run() {
    let mut list = CityList::new();

    loop {
        println!();
        println!("============== CITY LIST: SEQUENTIAL LIST ===============");
        println!("  1. Insert city at front");
        println!("  2. Insert city at index");
        println!("  3. Remove first city");
        println!("  4. Remove city at index");
        println!("  5. Update city at index");
        println!("  6. Find city by name");
        println!("  7. Find cities within distance of a point");
        println!("  8. Display all cities");
        println!("  0. Return to main menu");
        println!("=========================================================");

        match read_usize("Please enter your choice: ") {
            1 => {
                let city = read_city();

                match list.push_front(city) {
                    Ok(()) => println!("City inserted successfully."),
                    Err(ListError::DuplicateCityName) => {
                        println!("Error: a city with that name already exists.")
                    }
                    Err(ListError::IndexOutOfBounds) => unreachable!(),
                }
            }

            2 => {
                let index = read_usize("Index: ");
                let city = read_city();

                match list.insert_at(index, city) {
                    Ok(()) => println!("City inserted successfully."),
                    Err(ListError::DuplicateCityName) => {
                        println!("Error: a city with that name already exists.")
                    }
                    Err(ListError::IndexOutOfBounds) => {
                        println!("Error: index is out of bounds.")
                    }
                }
            }

            3 => match list.pop_front() {
                Some(city) => {
                    println!("Removed city:");
                    print_city(&city);
                }
                None => println!("The list is empty."),
            },

            4 => {
                let index = read_usize("Index: ");

                match list.remove_at(index) {
                    Some(city) => {
                        println!("Removed city:");
                        print_city(&city);
                    }
                    None => println!("Error: index is out of bounds."),
                }
            }

            5 => {
                let index = read_usize("Index: ");

                println!("Enter the new city information:");
                let city = read_city();

                match list.update_at(index, city) {
                    Ok(()) => println!("City updated successfully."),
                    Err(ListError::DuplicateCityName) => {
                        println!("Error: a city with that name already exists.")
                    }
                    Err(ListError::IndexOutOfBounds) => {
                        println!("Error: index is out of bounds.")
                    }
                }
            }

            6 => {
                let name = read_string("City name: ");

                match list.find_by_name(&name) {
                    Some(city) => {
                        println!("City found:");
                        print_city(city);
                    }
                    None => println!("No city named \"{name}\" was found."),
                }
            }

            7 => {
                println!("Enter the center point:");

                let x = read_f64("x: ");
                let y = read_f64("y: ");

                let max_distance = loop {
                    let distance = read_f64("Maximum distance: ");

                    if distance >= 0.0 {
                        break distance;
                    }

                    println!("Distance must be non-negative.");
                };

                let point = Point { x, y };
                let cities = list.cities_within_distance_of(&point, max_distance);

                if cities.is_empty() {
                    println!("No cities were found within that distance.");
                } else {
                    println!("Cities found:");

                    for (index, city) in cities.iter().enumerate() {
                        println!(
                            "  {}. {} ({}, {})",
                            index + 1,
                            city.name,
                            city.point.x,
                            city.point.y
                        );
                    }
                }
            }

            8 => {
                print_sequential_list(&list);
            }

            0 => break,

            _ => {
                println!("Invalid choice. Please enter a number from 0 to 8.");
            }
        }
    }
}

fn print_sequential_list(list: &CityList) {
    println!();
    println!("-------------------- CITY LIST --------------------");

    if list.city_data.is_empty() {
        println!("The list is empty.");
    } else {
        for (index, city) in list.city_data.iter().enumerate() {
            println!(
                "[{index}] {} ({}, {})",
                city.name, city.point.x, city.point.y
            );
        }
    }

    println!("---------------------------------------------------");
}

fn print_city(city: &City) {
    println!("{} ({}, {})", city.name, city.point.x, city.point.y);
}

fn read_city() -> City {
    let name = read_string("City name: ");
    let x = read_f64("x: ");
    let y = read_f64("y: ");

    City {
        name,
        point: Point { x, y },
    }
}

fn read_string(prompt: &str) -> String {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if !input.is_empty() {
                    return input.to_owned();
                }

                println!("Input cannot be empty.");
            }

            Err(error) => {
                eprintln!("Failed to read input: {error}");
            }
        }
    }
}

fn read_usize(prompt: &str) -> usize {
    loop {
        let input = read_string(prompt);

        match input.parse::<usize>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid non-negative integer."),
        }
    }
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_string(prompt);

        match input.parse::<f64>() {
            Ok(value) if value.is_finite() => return value,
            Ok(_) => println!("Please enter a finite number."),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // we need to test the following functions
    // City:
    //   within_distance_of
    #[test]
    fn test_city_distance_fn() {
        let test_city: City = City {
            name: String::from("Chongqing"),
            point: Point { x: 0f64, y: 0f64 },
        };
        let max_distance: f64 = 5.0;
        let point1 = Point { x: 2.0, y: 4.0 };
        let point2 = Point { x: 3.0, y: 4.0 };
        let point3 = Point { x: 3.0, y: 5.0 };
        assert!(test_city.within_distance_of(&point1, max_distance));
        assert!(test_city.within_distance_of(&point2, max_distance));
        assert!(!test_city.within_distance_of(&point3, max_distance));
    }
    // CityLinkedList:
    //   new
    //   push_front
    //   insert_at
    //   pop_front
    //   remove_at
    //   update_at
    //   find_by_name
    //   cities_within_distance_of
    #[test]
    fn test_linked_list() {
        let mut test_list = CityList::new();
        let chongqing: City = City {
            name: String::from("Chongqing"),
            point: Point { x: 0f64, y: 0f64 },
        };
        let sichuan: City = City {
            name: String::from("Sichuan"),
            point: Point { x: 12f64, y: 5f64 },
        };
        let shanghai: City = City {
            name: String::from("Shanghai"),
            point: Point { x: 3f64, y: 4f64 },
        };
        let beijing: City = City {
            name: String::from("Beijing"),
            point: Point { x: 7f64, y: 24f64 },
        };
        assert_eq!(Ok(()), test_list.push_front(chongqing.clone()));
        assert_eq!(test_list.pop_front(), Some(chongqing.clone()));
        assert_eq!(test_list.pop_front(), None);

        // only Chongqing
        assert_eq!(Ok(()), test_list.push_front(chongqing.clone()));
        assert_eq!(
            Err(ListError::DuplicateCityName),
            test_list.push_front(chongqing.clone())
        );
        assert_eq!(test_list.find_by_name("Chongqing"), Some(&chongqing));

        // Beijing, Chongqing
        assert_eq!(Ok(()), test_list.push_front(beijing.clone()));
        assert_eq!(test_list.find_by_name("Beijing"), Some(&beijing));

        // Shanghai, Beijing, Chongqing
        assert_eq!(Ok(()), test_list.push_front(shanghai.clone()));

        // Beijing, Chongqing
        assert_eq!(Some(shanghai.clone()), test_list.remove_at(0));
        assert_eq!(None, test_list.remove_at(2));

        // Beijing
        assert_eq!(Some(beijing.clone()), test_list.remove_at(0));

        // Chongqing
        assert_eq!(Some(chongqing.clone()), test_list.remove_at(0));

        // Nothing here!!

        assert_eq!(Ok(()), test_list.push_front(beijing.clone())); //3
        assert_eq!(Ok(()), test_list.push_front(chongqing.clone())); //2
        assert_eq!(Ok(()), test_list.push_front(sichuan.clone())); //1
        assert_eq!(Ok(()), test_list.push_front(shanghai.clone())); //0

        assert_eq!(
            test_list.cities_within_distance_of(&Point { x: 0f64, y: 0f64 }, 13f64),
            vec![&shanghai, &sichuan, &chongqing]
        );

        let chongqing_1 = City {
            name: String::from("Chongqing1"),
            ..chongqing.clone()
        };
        let beijing_1 = City {
            name: String::from("Beijing1"),
            ..beijing.clone()
        };

        let _ = test_list.update_at(2, chongqing_1.clone());
        let _ = test_list.update_at(3, beijing_1.clone());

        assert_eq!(
            test_list.cities_within_distance_of(&Point { x: 0f64, y: 0f64 }, 13f64),
            vec![&shanghai, &sichuan, &chongqing_1]
        );
    }
}
