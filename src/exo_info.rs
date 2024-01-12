
use serde::Deserialize;
use yew::prelude::*;

const FLOAT_DEFAULT : f64 = -1.0 as f64;

fn float_default() -> f64 {
    FLOAT_DEFAULT
}


#[derive(Deserialize, Clone, PartialEq)]
pub struct Planet{
    pl_name: String,
    pl_bmassj: Option<f64>,
    pl_rade: Option<f64>,
    pl_insol: Option<f64>,
}

#[derive(Properties, PartialEq)]
pub struct PlanetListProps{
    pub planets: Vec<Planet>,
}




#[function_component(PlanetList)]
pub fn planet_list(PlanetListProps {planets}: &PlanetListProps) -> Html {
    planets.iter().map(|planet| html! {
        <p key={planet.pl_name.clone()}>{format!("Name:{}, Mass:{}, Radius:{}, Temperature:{}", planet.pl_name, 
            match planet.pl_bmassj {
                Some(mass) => mass,
                None => 0.0,
            }, 
            match planet.pl_rade {
                Some(radius) => radius,
                None => 0.0,
            },
            match planet.pl_insol {
                Some(insolation) => insolation,
                None => 0.0,
            })}</p>

    }).collect()
}