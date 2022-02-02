use crate::data::{Race, WoWClass};
use crate::{get_random_class, get_random_race};
use std::rc::Rc;
use yew::Reducible;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Filter {
    Alliance,
    Horde,
}

#[derive(Clone, Debug)]
pub struct State {
    pub faction_filter: Option<Filter>,
    pub selected_race: Race,
    pub selected_class: WoWClass,
}

impl Default for State {
    fn default() -> Self {
        let selected_race = get_random_race();

        Self {
            faction_filter: None,
            selected_race: selected_race.clone(),
            selected_class: get_random_class(&selected_race),
        }
    }
}

#[allow(dead_code)]
pub enum Action {
    Reset,
    SetFilter(Option<Filter>),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Reset => {
                let selected_race = get_random_race();

                // TODO: Implement filter.

                State {
                    faction_filter: self.faction_filter.clone(),
                    selected_race: selected_race.clone(),
                    selected_class: get_random_class(&selected_race),
                }
                .into()
            }
            Action::SetFilter(filter) => State {
                faction_filter: filter,
                selected_race: self.selected_race.clone(),
                selected_class: self.selected_class.clone(),
            }
            .into(),
        }
    }
}
