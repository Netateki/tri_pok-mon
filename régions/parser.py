import json
import glob
import os
import re

def parse_types(type_string):
    types_raw = type_string.split('/')
    replacements = {"Électrik": "Electrik", "Fée": "Fee", "Ténèbres": "Tenebres"}
    return [replacements.get(t.strip(), t.strip()) for t in types_raw]

def determine_category(tag):
    tag = tag.strip().lower()
    if tag == "starter":
        return "Starter"
    elif tag in ["légendaire", "legendaire"]:
        return "Legendaire"
    elif tag == "fossile":
        return "Fossile"
    elif tag in ["éthologique", "ethologique"]:
        return "Ethologique"
    return "Normal"

def main():
    pokedex = {}

    for filepath in glob.glob("*.txt"):
        region_name = os.path.splitext(os.path.basename(filepath))[0].capitalize()
        
        with open(filepath, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line or ":" not in line:
                    continue
                
                location_part, pokemons_part = line.split(":", 1)
                location_raw = location_part.strip()
                
                # Création d'une clé d'espace de nom unique pour le lieu
                location_name = f"{location_raw} ({region_name})"
                
                matches = re.findall("([A-Za-zÀ-ÿ0-9♂♀\\s\\-\\.']+?)\\s*\\(([^)]+)\\)(?:\\s*\\[([^\\]]+)\\])?", pokemons_part)
                
                for match in matches:
                    poke_name = match[0].strip()
                    poke_types = match[1].strip()
                    poke_tag = match[2].strip() if len(match) > 2 else ""
                    
                    if poke_name not in pokedex:
                        pokedex[poke_name] = {
                            "name": poke_name,
                            "category": determine_category(poke_tag),
                            "types": parse_types(poke_types),
                            "locations": [location_name],
                            "regions": [region_name]  # BASCULE EN LISTE (avec un 's')
                        }
                    else:
                        if location_name not in pokedex[poke_name]["locations"]:
                            pokedex[poke_name]["locations"].append(location_name)
                        # On ajoute la région si elle n'est pas déjà connue pour ce Pokémon
                        if region_name not in pokedex[poke_name]["regions"]:
                            pokedex[poke_name]["regions"].append(region_name)

    final_list = list(pokedex.values())

    with open("pokedex.json", "w", encoding="utf-8") as out:
        json.dump(final_list, out, indent=4, ensure_ascii=False)
        
    print(f"Extraction terminée : {len(final_list)} Pokémon uniques traités et compilés dans pokedex.json.")

if __name__ == "__main__":
    main()