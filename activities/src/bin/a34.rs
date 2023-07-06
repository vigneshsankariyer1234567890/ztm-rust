// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct Luggage<State> {
  tracking_id: i32,
  state: State,
}

impl<State> Luggage<State> {
  fn transition<NewState>(self, new_state: NewState) -> Luggage<NewState> {
    Luggage {
      tracking_id: self.tracking_id,
      state: new_state
    }
  }
}

struct CheckIn;
struct OnLoading;
struct OffLoading;
struct AwaitingPickup;
struct EndCustody;

impl Luggage<CheckIn> {
  fn load_on_plane(self) -> Luggage<OnLoading> {
    self.transition(OnLoading)
  }
}

impl Luggage<OnLoading> {
  fn take_off_plane(self) -> Luggage<OffLoading> {
    self.transition(OffLoading)
  }
}

impl Luggage<OffLoading> {
  fn await_pickup(self) -> Luggage<AwaitingPickup> {
    self.transition(AwaitingPickup)
  }
}

impl Luggage<AwaitingPickup>  {
  fn hand_custody(self) -> Luggage<EndCustody> {
    self.transition(EndCustody)
  }
}


fn main() {
  let checked_in_bag = Luggage {
    tracking_id: 1000,
    state: CheckIn
  };

  checked_in_bag.load_on_plane().take_off_plane().await_pickup().hand_custody();
}
