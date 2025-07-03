
mod fixture;
mod universe;



#[cfg(test)]
mod tests {
    use std::{fs::File, future, io::Write};

    use crate::{fixture::{Color, Dimmer, Fixture}, universe::Universe};

    use super::*;

    #[test]
    fn gen_dmx_universe(){
        let mut universe = Universe::new();

        let mut led = Fixture::new(0, 1);

        led.add_component(fixture::FixtureComponent::Color(Color{r:255,g:03,b:100}));
        led.add_component(fixture::FixtureComponent::Dimmer(Dimmer { intensity: 50 }));

        universe.add_fixture(led);

        let mut file = File::create("test_universe.json").expect("Couldnt open file");
        file.write_all(universe.export_to_json().as_bytes()).expect("Couldnt write to file");
        dbg!(universe.get_dmx_values());
    }
}
