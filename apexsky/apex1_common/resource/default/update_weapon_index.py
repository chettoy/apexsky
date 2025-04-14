# Author: 🦅 | Co-Author: ChatGPT
# Date: April 14, 2025

import json

# Load original and updated weapon JSONs
with open('weapon.json') as f:
    old_weapon_list = json.load(f)

with open('updated_weapon.json') as f:
    new_weapon_list = json.load(f)

# Original WeaponId enum
weapon_enum_raw = """
R301 = 0,
Sentinel = 2,
Bow = 3,
R2R5 = 4,
Rampage = 7,
Alternator = 84,
AlternatorDayzero = 85,
Re45 = 86,
Re45Crate = 87,
Re45Dayzero = 88,
ChargeRifle = 89,
ChargeRifleCe = 90,
Devotion = 91,
DevotionCrate = 92,
DevotionDayzero = 93,
Longbow = 94,
LongbowDayzero = 95,
Havoc = 96,
HavocDayzero = 97,
HavocCrate = 98,
Eva8 = 99,
Eva8Crate = 100,
Eva8Dayzero = 101,
Flatline = 102,
FlatlineDayzero = 103,
G7Scout = 104,
G7ScoutDayzero = 105,
Hemlock = 106,
HermlockCrate = 107,
HemlockDayzero = 108,
Kraber = 109,
KraberDayzero = 110,
Lstar = 111,
LstarCrate = 112,
LstarDayzero = 113,
Mastiff = 114,
MastiffDayzero = 115,
MastiffCrate = 116,
Mozambique = 117,
MozambiqueLight = 118,
MozambiqueEnergy = 119,
MozambiqueSniper = 120,
MozambiqueHeavy = 121,
MozambiqueDayzero = 122,
Prowler = 123,
ProwlerCrate = 124,
ProwlerDayzero = 125,
Peacekeeper = 126,
PeacekeeperDayzero = 127,
PeacekeeperCrate = 128,
R301Dayzero = 129,
R99 = 130,
R99Dayzero = 131,
R99Crate = 132,
P2020 = 133,
P2020Dayzero = 134,
Spitfire = 135,
SpitfireDayzero = 136,
TripleTake = 137,
TripleTakeDayzero = 138,
Wingman = 139,
WingmanCrate = 140,
WingmanDayzero = 141,
Volt = 142,
_3030Repeater = 143,
CarSmg = 144,
Nemesis = 145,
GrenadeFrag = 146,
GrenadeArcStar = 147,
GrenadeThermite = 148,
Hands = 149,
Epg = 200,
EpgTethered = 201,
ThrowingKnife = 202,
"""

# Map display names to old indices from old_weapon_list
old_weapon_index_map = {name: idx for idx, name in enumerate(old_weapon_list)}
new_weapon_index_map = {name: idx for idx, name in enumerate(new_weapon_list)}

# Map enum names to weapon names (reverse lookup based on index)
weapon_name_by_enum = {}
for line in weapon_enum_raw.strip().splitlines():
    if '=' in line:
        name, index = line.strip().strip(',').split('=')
        index = int(index.strip())
        name = name.strip()
        weapon_name = next((k for k, v in old_weapon_index_map.items() if v == index), None)
        if weapon_name:
            weapon_name_by_enum[name] = weapon_name

# Build updated enum
updated_enum_lines = ["pub enum WeaponId {"]
for enum_name, old_weapon in weapon_name_by_enum.items():
    if old_weapon in new_weapon_index_map:
        new_index = new_weapon_index_map[old_weapon]
        updated_enum_lines.append(f"    {enum_name} = {new_index},")
    else:
        # Weapon no longer exists
        updated_enum_lines.append(f"    // {enum_name} = (was {old_weapon_index_map[old_weapon]}), removed")

updated_enum_lines.append("    Max,")
updated_enum_lines.append("}")

# Output the updated enum
print("\n".join(updated_enum_lines))
