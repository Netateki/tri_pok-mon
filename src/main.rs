mod models;

use models::*;
use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement};

#[function_component(App)]
fn app() -> Html {
    let poke_list = use_state(|| {
        let json_data = include_str!("../pokedex.json");
        let parsed_data: Vec<Pokemon> = serde_json::from_str(json_data).expect("Erreur JSON");
        parsed_data
    });

    let search_name = use_state(|| String::new());
    let search_location = use_state(|| String::new());
    
    // On initialise directement avec les valeurs par défaut des menus
    let search_type = use_state(|| String::from("Tous"));
    let search_region = use_state(|| String::from("Toutes"));
    let search_category = use_state(|| String::from("Toutes"));

    let sort_alpha = {
        let poke_list = poke_list.clone();
        Callback::from(move |_| {
            let mut current_list = (*poke_list).clone();
            current_list.sort_by(|a, b| a.name.cmp(&b.name));
            poke_list.set(current_list);
        })
    };

    let sort_category = {
        let poke_list = poke_list.clone();
        Callback::from(move |_| {
            let mut current_list = (*poke_list).clone();
            current_list.sort_by(|a, b| a.category.cmp(&b.category));
            poke_list.set(current_list);
        })
    };

    let on_name_input = {
        let search_name = search_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_name.set(input.value().to_lowercase());
        })
    };

    let on_type_change = {
        let search_type = search_type.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            search_type.set(select.value());
        })
    };
    
    let on_region_change = {
        let search_region = search_region.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            search_region.set(select.value());
        })
    };

    let on_location_input = {
        let search_location = search_location.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_location.set(input.value().to_lowercase());
        })
    };

    let on_category_change = {
        let search_category = search_category.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            search_category.set(select.value());
        })
    };

    // Extraction pour Autocomplétion (Lieux)
    let mut unique_locations: Vec<String> = poke_list.iter().flat_map(|p| p.locations.clone()).collect();
    unique_locations.sort();
    unique_locations.dedup();

    // Extraction pour Menu Déroulant (Régions)
    let mut unique_regions: Vec<String> = poke_list.iter().map(|p| p.region.clone()).collect();
    unique_regions.sort();
    unique_regions.dedup();

    // Filtre croisé à 5 dimensions maintenant
    let filtered_pokemons: Vec<Pokemon> = poke_list.iter().filter(|p| {
        let match_name = search_name.is_empty() || p.name.to_lowercase().contains(&*search_name);
        
        let match_type = search_type.is_empty() || search_type.as_str() == "Tous" || p.types.iter().any(|t| format!("{:?}", t) == *search_type);
            
        let match_region = search_region.is_empty() || search_region.as_str() == "Toutes" || p.region == *search_region;

        let match_location = search_location.is_empty() || p.locations.iter().any(|loc| loc.to_lowercase().contains(&*search_location));

        let match_category = search_category.is_empty() || search_category.as_str() == "Toutes" || format!("{:?}", p.category) == *search_category;
        
        match_name && match_type && match_region && match_location && match_category
    }).cloned().collect();

    let all_types = vec!["Tous", "Eau", "Poison", "Insecte", "Plante", "Normal", "Vol", "Electrik", "Roche", "Sol", "Psy", "Fee", "Combat", "Feu", "Glace", "Spectre", "Dragon", "Acier", "Tenebres"];
    let all_categories = vec!["Toutes", "Normal", "Starter", "Legendaire", "Fossile", "Ethologique"];

    html! {
        <div style="padding: 20px; font-family: sans-serif;">
            <h1> { "Moteur de tri Pokémon" } </h1>
            
            <div style="margin-bottom: 20px; display: flex; gap: 10px; align-items: center; flex-wrap: wrap;">
                
                <input type="text" placeholder="Chercher un nom..." oninput={on_name_input} style="padding: 5px; width: 150px;"/>
                
                // Select Catégorie
                <select onchange={on_category_change} style="padding: 5px;">
                    { all_categories.iter().map(|c| html! { 
                        <option value={*c} selected={*c == search_category.as_str()}>
                            { if *c == "Toutes" { "Toutes Catégories" } else { c } }
                        </option> 
                    }).collect::<Html>() }
                </select>

                // Select Type
                <select onchange={on_type_change} style="padding: 5px;">
                    { all_types.iter().map(|t| html! { 
                        <option value={*t} selected={*t == search_type.as_str()}>
                            { t }
                        </option> 
                    }).collect::<Html>() }
                </select>

                // Select Région
                <select onchange={on_region_change} style="padding: 5px;">
                    <option value="Toutes" selected={"Toutes" == search_region.as_str()}>{ "Toutes Régions" }</option>
                    {
                        unique_regions.iter().map(|r| {
                            html! { 
                                <option value={r.clone()} selected={r.as_str() == search_region.as_str()}>
                                    { r }
                                </option> 
                            }
                        }).collect::<Html>()
                    }
                </select>

                <input type="text" placeholder="Chercher un lieu..." list="locations-list" oninput={on_location_input} style="padding: 5px; width: 150px;"/>
                <datalist id="locations-list">
                    { unique_locations.iter().map(|loc| html! { <option value={loc.clone()} /> }).collect::<Html>() }
                </datalist>

                <button onclick={sort_alpha} style="padding: 5px;"> { "Trier A-Z" } </button>
                <button onclick={sort_category} style="padding: 5px;"> { "Trier par Catégorie" } </button>
            </div>

            <ul>
                {
                    filtered_pokemons.iter().map(|p| {
                        html! { 
                            <li style="margin-bottom: 5px;"> 
                                <strong> { format!("{}", p.name) } </strong> 
                                { format!(" - {:?} | Région: {} | Types: {:?} | Lieux: {}", p.category, p.region, p.types, p.locations.join(", ")) } 
                            </li> 
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}