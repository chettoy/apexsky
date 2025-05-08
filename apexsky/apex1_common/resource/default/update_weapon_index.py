# Author: 🦅 | Co-Author: ChatGPT
# Date: April 14, 2025

import json

# Load original and updated weapon JSONs
with open("weapon.json") as f:
    old_weapon_list = json.load(f)

with open("updated_weapon.json") as f:
    new_weapon_list = json.load(f)

# Original WeaponId enum
weapon_enum_raw = """
R301 = 0,
Sentinel = 1,
Bow = 2,
R2R5 = 3,
Rampage = 6,
Alternator = 83,
Re45 = 84,
Re45Crate = 85,
ChargeRifle = 86,
Devotion = 87,
DevotionCrate = 88,
Longbow = 89,
Havoc = 90,
HavocCrate = 91,
Eva8 = 92,
Eva8Crate = 93,
Flatline = 94,
G7Scout = 95,
Hemlock = 96,
HermlockCrate = 97,
Kraber = 98,
Lstar = 99,
LstarCrate = 100,
Mastiff = 101,
MastiffCrate = 102,
Mozambique = 103,
MozambiqueLight = 104,
MozambiqueEnergy = 105,
MozambiqueSniper = 106,
MozambiqueHeavy = 107,
Prowler = 108,
ProwlerCrate = 109,
Peacekeeper = 110,
PeacekeeperCrate = 111,
R99 = 112,
R99Crate = 113,
P2020 = 114,
Spitfire = 115,
TripleTake = 116,
Wingman = 117,
WingmanCrate = 118,
Volt = 119,
_3030Repeater = 120,
CarSmg = 121,
Nemesis = 122,
GrenadeFrag = 123,
GrenadeArcStar = 124,
GrenadeThermite = 125,
Hands = 126,
Epg = 180,
EpgTethered = 181,
ThrowingKnife = 182,
"""

# Map display names to old indices from old_weapon_list
old_weapon_index_map = {name: idx for idx, name in enumerate(old_weapon_list)}
new_weapon_index_map = {name: idx for idx, name in enumerate(new_weapon_list)}

# Map enum names to weapon names (reverse lookup based on index)
weapon_name_by_enum = {}
for line in weapon_enum_raw.strip().splitlines():
    if "=" in line:
        name, index = line.strip().strip(",").split("=")
        index = int(index.strip())
        name = name.strip()
        weapon_name = next(
            (k for k, v in old_weapon_index_map.items() if v == index), None
        )
        if weapon_name:
            weapon_name_by_enum[name] = weapon_name
            print(f"{name}: {weapon_name}")
print()

# Build updated enum
updated_enum_lines = ["pub enum WeaponId {"]
for enum_name, old_weapon in weapon_name_by_enum.items():
    if old_weapon in new_weapon_index_map:
        new_index = new_weapon_index_map[old_weapon]
        updated_enum_lines.append(f"    {enum_name} = {new_index},")
    else:
        # Weapon no longer exists
        updated_enum_lines.append(
            f"    // {enum_name} = (was {old_weapon_index_map[old_weapon]}), removed"
        )

updated_enum_lines.append("    Max,")
updated_enum_lines.append("}")

# Output the updated enum
print("\n".join(updated_enum_lines))
