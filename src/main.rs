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
        <div>
            <div class="controls-container">
                <h2 style="margin: 0 15px 0 0;"> { "MasterDex" } </h2>
                
                <input type="text" placeholder="Chercher un nom..." oninput={on_name_input} />
                
                <select onchange={on_category_change}>
                    { all_categories.iter().map(|c| html! { 
                        <option value={*c} selected={*c == search_category.as_str()}>
                            { if *c == "Toutes" { "Toutes Catégories" } else { c } }
                        </option> 
                    }).collect::<Html>() }
                </select>

                <select onchange={on_type_change}>
                    { all_types.iter().map(|t| html! { 
                        <option value={*t} selected={*t == search_type.as_str()}>
                            { t }
                        </option> 
                    }).collect::<Html>() }
                </select>

                <select onchange={on_region_change}>
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

                <input type="text" placeholder="Chercher un lieu..." list="locations-list" oninput={on_location_input} />
                <datalist id="locations-list">
                    { unique_locations.iter().map(|loc| html! { <option value={loc.clone()} /> }).collect::<Html>() }
                </datalist>

                <button onclick={sort_alpha}> { "Trier A-Z" } </button>
                <button onclick={sort_category}> { "Trier par Catégorie" } </button>
            </div>

            <ul class="poke-grid">
                {
                    filtered_pokemons.iter().map(|p| {
                        html! { 
                            <li class="poke-card"> 
                                <div class="poke-name">{ format!("{}", p.name) }</div>
                                
                                <div class="badges">
                                    // Le format Rust génère dynamiquement la classe CSS (ex: "cat-Starter")
                                    <span class={format!("badge cat-{:?}", p.category)}>
                                        { format!("{:?}", p.category) }
                                    </span>
                                    <span class="badge badge-region">{ format!("{}", p.region) }</span>
                                    
                                    {
                                        p.types.iter().map(|t| html! {
                                            <span class="badge badge-type">{ format!("{:?}", t) }</span>
                                        }).collect::<Html>()
                                    }
                                </div>

                                <div class="badges">
                                    {
                                        p.locations.iter().map(|loc| html! {
                                            <span class="badge badge-location">{ loc }</span>
                                        }).collect::<Html>()
                                    }
                                </div>
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