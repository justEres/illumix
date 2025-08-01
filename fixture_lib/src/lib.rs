pub mod fixture;
pub mod universe;
pub mod patching;
pub mod networking;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{
        fixture::{Color, Dimmer, Fixture, FixtureComponent}, networking::Packet, patching::Patching, universe::Universe
    };

    use super::*;

    #[test]
    fn gen_dmx_universe() {
        let mut universe = Universe::new();

        let mut led = Fixture::new(0, 1, "Led".into());

        led.add_component(fixture::FixtureComponent::Color(Color {
            r: 255,
            g: 03,
            b: 100,
        }));
        led.add_component(fixture::FixtureComponent::Dimmer(Dimmer { intensity: 50 }));

        universe.add_fixture(led);

        let mut file = File::create("test_universe.json").expect("Couldnt open file");
        file.write_all(universe.export_to_json().as_bytes())
            .expect("Couldnt write to file");
        dbg!(universe.get_dmx_values());
    }

    #[test]
    fn gen_patching() {
        let fixture = patching::FixturePreset{ name: "1kw".into(), components: vec![FixtureComponent::Dimmer(Dimmer { intensity: 0 })]};

        fixture.store_to_file("1kw.json".into());



        let mut patching = Patching::new();
        patching.fixtures.push(patching::Fixture{ id: 0, dmx_address: 12, fixture_preset: "1kw.json".into()});
        patching.store_to_file("patching.json".into());

        dbg!(patching.to_universe());

        


    }


    #[test]
    fn temp(){
        let mut fixture = Fixture::new(1, 12, "leck mich im arsch".into());
        fixture.add_component(FixtureComponent::Placeholder);

        println!("{}",serde_json::to_string(&fixture).unwrap());
    }

    #[test]
    fn req_full_uni_test(){
        let p = Packet{
            packet_type: networking::PacketType::RequestFullUniverse
        };
        println!("{}",String::from_utf8(p.serialize()).unwrap());
    }
}
