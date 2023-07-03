use std::fmt;

#[derive(Clone)]
pub struct Bill {
  pub(crate) name: String,
  pub(crate) amount_in_dollars: i32,
  pub(crate) amount_in_cents: i32,
}

impl Bill {
  pub fn new(name: &str, amount_in_dollars: i32, amount_in_cents: i32) -> Option<Self> {
    Some(
      Self {
        name: name.to_string(),
        amount_in_cents,
        amount_in_dollars,
      }
    )
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_amount_in_cents(&self) -> i32 {
    self.amount_in_cents
  }

  pub fn get_amount_in_dollars(&self) -> i32 {
    self.amount_in_dollars
  }

  fn generate_amount_for_bill(input_array: &Vec<String>) -> Option<(i32, i32)> {
    if input_array.len() != 2 {
      return None;
    }

    let amount_in_dollars = input_array[0].parse::<i32>().ok()?;
    let amount_in_cents = input_array[1].parse::<i32>().ok()?;

    Some((amount_in_dollars, amount_in_cents))
  }

  pub fn generate_bill(name: &str, input_array: &Vec<String>) -> Option<Bill> {
    let (
      amount_in_dollars,
      amount_in_cents
    ) = Bill::generate_amount_for_bill(input_array)?;

    Bill::new(name, amount_in_dollars, amount_in_cents)
  }
}

impl fmt::Debug for Bill {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{{
      Name => {:?},
      Amount => ${:?}.{:?}
    }},", self.name, self.amount_in_dollars, self.amount_in_cents)
  }
}