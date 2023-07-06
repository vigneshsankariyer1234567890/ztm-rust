// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Person<'a> {
  id: &'a str,
  first_name: &'a str,
  email: &'a str,
  dept: &'a str,
  title: &'a str,
}

impl<'a> Person<'a> {
  fn new(data: Vec<&'a str>) -> Self {
    Self {
      id: &data[0],
      first_name: &data[1],
      email: &data[2],
      dept: &data[3],
      title: &data[4],
    }
  }

  fn get_name(&self) -> &str {
    self.first_name
  }

  fn get_title(&self) -> &str {
    self.title
  }
}

struct People<'a> {
  inner: Vec<Person<'a>>
}

impl<'a> People<'a> {
  fn of(person_list: Vec<Person<'a>>) -> Self {
    Self {
      inner: person_list
    }
  }

  fn display_names_and_tiles(&self) {
    for person in &self.inner {
      println!("Name: {:?}, Job: {:?}", person.get_name(), person.get_title())
    }
  }
}


fn main() {
  let person_strs: Vec<&str> = MOCK_DATA.trim().split('\n').collect();

  let person_vec: Vec<Person> = person_strs.iter()
    .map(|s| {
      let split: Vec<&str> = s.split(',').collect();
      
      Person::new(split)
    })
    .collect();

  let people: People = People::of(person_vec);

  people.display_names_and_tiles();
}
