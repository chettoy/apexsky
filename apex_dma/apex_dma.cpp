#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <string.h>
#include <random>
#include <chrono>
#include <iostream>
#include <cfloat>
#include "Game.h"
#include <thread>

//this is a test, with seconds
Memory apex_mem;


//Just setting things up, dont edit.
bool active = true;
uintptr_t aimentity = 0;
uintptr_t tmp_aimentity = 0;
uintptr_t lastaimentity = 0;
float max = 9999.0f;
int team_player = 0;
const int toRead = 100;


bool aiming = false;



float smoothpred = 0.08;
float smoothpred2 = 0.05;


//CONFIG AREA, you must set all the true/false to what you want.
bool item_glow = true;
bool player_glow = true;
bool aim_no_recoil = true;
float max_fov = 15;
int aim = 2;
bool firing_range = false;
int bone = 2;
extern float smooth; //Config is in Game.cpp, Line 15 min 85 no beaming, 100 somewhat beam people, 125 should be safe

//ITEM GLOW TOGGLES
int itemglowbrightness = 8; //10 is none and 0 is full glow like the sun in your eye's.
//Backpacks
bool lightbackpack = false;
bool medbackpack = true;
bool heavybackpack = true;


//Shield upgrades
bool shieldupgrade = true;
bool shieldupgradehead = true;
bool shielddown = true;


//heaing and Misc
bool accelerant = false;
bool phoenix = false;
bool healthlarge = true;
bool healthsmall = false;
bool shieldbattsmall = false;
bool shieldbattlarge = true;


//Ammo
bool ammosniper = true;
bool ammohc = true;
bool ammosc = true;
bool ammonrg = true;
bool ammoshotgun = false;


//Optics
bool optic = false;
bool optic2x = true;
bool opticholo1x = false;
bool opticholo1x2x = true;
bool opticthreat = true;
bool optic3x = true;
bool optic2x4x = true;
bool opticsniper6x = true;
bool opticsniper4x8x = true;
bool opticsniperthreat = false;


//Magazines
bool magsniper = true;
bool magenergy = true;
bool lightammomag = true;
bool heavyammomag = true;


//Attachments 
bool lasersight = true;
bool stocksniper = true;
bool stockregular = true;
bool suppressor = true;
bool weaponmod = false;
bool shotgunbolt = false;


//Nades
bool grenade_frag = false;
bool grenade_arc_star = false;
bool grenade_thermite = false;


//Kraber
bool weapon_kraber = true;


//Shotguns
bool weapon_mastiff = false;
bool weapon_eva8  = false;
bool weapon_peacekeeper  = false;
bool weapon_mozambique  = false;


//Energy weapons
bool weapon_lstar = true;
bool weapon_nemesis = true;
bool weapon_havoc = true;
bool weapon_devotion = false;
bool weapon_triple_take = true;
bool weapon_prowler  = false;
bool weapon_volt  = false;


//Heavy Weapons
bool weapon_flatline = true;
bool weapon_hemlock  = false;
bool weapon_3030_repeater = false; 
bool weapon_rampage  = true;
bool weapon_car_smg  = true;


//Light weapons
bool weapon_p2020  = false;
bool weapon_re45  = false;
bool weapon_g7_scout  = false;
bool weapon_alternator  = false;
bool weapon_r99  = true;
bool weapon_spitfire  = false;
bool weapon_r301 = true;


//Snipers.. wingman is the odd one...and the bow..
bool weapon_wingman  = true;
bool weapon_longbow  = false;
bool weapon_charge_rifle  = false;
bool weapon_sentinel  = false;
bool weapon_bow  = false;




//Player Glow Color and Brightness. Edit the RGB
//not visable
float glowr = 192.0f; //Red 0-255, higher is brighter color.
float glowg = 0.0f; //Green 0-255, higher is brighter color.
float glowb = 0.0f; //Blue 0-255, higher is brighter color.
//visable
float glowrviz = 0.0f; //Red 0-255, higher is brighter color.
float glowgviz = 192.0f; //Green 0-255, higher is brighter color.
float glowbviz = 0.0f; //Blue 0-255, higher is brighter color.
//knocked
float glowrknocked = 158.0f; //Red 0-255, higher is brighter color.
float glowgknocked = 158.0f; //Green 0-255, higher is brighter color.
float glowbknocked = 158.0f; //Blue 0-255, higher is brighter color.


//Removed but not all the way, dont edit.
int glowtype = 1;
int glowtype2 = 2;




//aim dist check. Just setting things up, dont edit.
float aimdist = 200.0f * 40.0f;




//Just setting things up, dont edit.
bool actions_t = false;
bool cactions_t = false;
bool esp_t = false;
bool aim_t = false;
bool vars_t = false;
bool item_t = false;
uint64_t g_Base;
bool next2 = false;
bool valid = false;
bool lock = false;



//Player Definitions, dont edit unless you know what you are doing.
typedef struct player
{
	float dist = 0;
	int entity_team = 0;
	bool knocked = false;
	bool visible = false;
	int health = 0;
	int shield = 0;
}player;


//Your in the matrix neo.
struct Matrix
{
	float matrix[16];
};


//Visual check and aim check.?
float lastvis_aim[toRead];


//Specator stuff. Just setting things up, dont edit.
int tmp_spec = 0, spectators = 0;
int tmp_all_spec = 0, allied_spectators = 0;



void MapRadarTesting()
{
	uintptr_t pLocal;
		apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, pLocal);
	int dt;
		apex_mem.Read<int>(pLocal + OFFSET_TEAM, dt);

	for (uintptr_t i = 0; i <= 80000; i++)
	{
		apex_mem.Write<int>(pLocal + OFFSET_TEAM, 1);
	}

	for (uintptr_t i = 0; i <= 80000; i++)
	{
		apex_mem.Write<int>(pLocal + OFFSET_TEAM, dt);
	}
}


//////////////////////////////////////////////////////////////////////////////////////////////////




uint64_t PlayerLocal;
int PlayerLocalTeamID;
int EntTeam;
int LocTeam;

using Clock = std::chrono::steady_clock;
std::chrono::time_point<std::chrono::steady_clock> start1, now1;
std::chrono::milliseconds duration1;

void loop()
{
	start1 = Clock::now();
}

std::chrono::steady_clock::time_point tduckStartTime;
bool mapRadarTestingEnabled = true;

void ClientActions()
{
	cactions_t = true;
	while (cactions_t)
	{
		std::this_thread::sleep_for(std::chrono::milliseconds(1));
		while (g_Base!=0)
		{
			
			uint64_t LocalPlayer = 0;
			apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);
			Entity LPlayer = getEntity(LocalPlayer);
			uint64_t entitylist = g_Base + OFFSET_ENTITYLIST;
			int attackState = 0;
			apex_mem.Read<int>(g_Base + OFFSET_IN_ATTACK, attackState); //108
			int tduckState = 0;
			apex_mem.Read<int>(g_Base + OFFSET_IN_TOGGLE_DUCK, tduckState); //61			
			int zoomState = 0;
			apex_mem.Read<int>(g_Base + OFFSET_IN_ZOOM, zoomState); //109
			//printf("%i\n", aiming);
			
			
			if (attackState == 108 || zoomState == 109)
			{
				aiming = true;
			}
			else
			{
				aiming = false;
			}
			
			
			
			now1 = Clock::now();
			duration1 = std::chrono::duration_cast<std::chrono::milliseconds>(now1 - start1);
			
						
			
			
		
				
			// Toggle crouch = check for ring
			if (attackState != 108 && tduckState == 61)
			{
				if (mapRadarTestingEnabled)
				{
					MapRadarTesting();
				}

				if (tduckStartTime == std::chrono::steady_clock::time_point())
				{
					tduckStartTime = std::chrono::steady_clock::now();
				}

				auto currentTime = std::chrono::steady_clock::now();
				auto duration = std::chrono::duration_cast<std::chrono::seconds>(currentTime - tduckStartTime).count();

				if (duration >= 3)
				{
					mapRadarTestingEnabled = false;
				}
			}
			else
			{
				tduckStartTime = std::chrono::steady_clock::time_point();
				mapRadarTestingEnabled = true;
			}
							
			std::this_thread::sleep_for(std::chrono::milliseconds(1));
		}
	}
	cactions_t = false;
}

void ProcessPlayer(Entity& LPlayer, Entity& target, uint64_t entitylist, int index)
{
	int entity_team = target.getTeamId();

	if (!target.isAlive())
	{
		float localyaw = LPlayer.GetYaw();
		float targetyaw = target.GetYaw();

		if(localyaw==targetyaw)
		{
			if(LPlayer.getTeamId() == entity_team)
				tmp_all_spec++;
			else
				tmp_spec++;
		}
		return;
	}
	

	Vector EntityPosition = target.getPosition();
	Vector LocalPlayerPosition = LPlayer.getPosition();
	float dist = LocalPlayerPosition.DistTo(EntityPosition);
	if (dist > aimdist) return;
	
	
	//Firing range stuff
	if(!firing_range)
		if (entity_team < 0 || entity_team>50 || entity_team == team_player) return;
	
	
	//Vis check aiming? dunno
	if(aim==2)
	{
		if((target.lastVisTime() > lastvis_aim[index]))
		{
			float fov = CalculateFov(LPlayer, target);
			if (fov < max)
			{
				max = fov;
				tmp_aimentity = target.ptr;
			}
		}
		else
		{
			if(aimentity==target.ptr)
			{
				aimentity=tmp_aimentity=lastaimentity=0;
			}
		}
	}
	else
	{
		float fov = CalculateFov(LPlayer, target);
		if (fov < max)
		{
			max = fov;
			tmp_aimentity = target.ptr;
		}
	}
	lastvis_aim[index] = target.lastVisTime();
}


//Main stuff, dont edit.
void DoActions()
{
	actions_t = true;
	while (actions_t)
	{
		std::this_thread::sleep_for(std::chrono::milliseconds(1));
		uint32_t counter = 0;

		while (g_Base!=0)
		{
			std::this_thread::sleep_for(std::chrono::milliseconds(30));	

			uint64_t LocalPlayer = 0;
			apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);
			if (LocalPlayer == 0) continue;

			Entity LPlayer = getEntity(LocalPlayer);

			team_player = LPlayer.getTeamId();
			if (team_player < 0 || team_player>50)
			{
				continue;
			}

			

			uint64_t entitylist = g_Base + OFFSET_ENTITYLIST;

			uint64_t baseent = 0;
			apex_mem.Read<uint64_t>(entitylist, baseent);
			if (baseent == 0)
			{
				continue;
			}

			max = 999.0f;
			tmp_aimentity = 0;
			tmp_spec = 0;
			tmp_all_spec = 0;
			if(firing_range)
			{
				int c=0;
				for (int i = 0; i < 16000; i++)
				{
					uint64_t centity = 0;
					apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
					if (centity == 0) continue;
					if (LocalPlayer == centity) continue;

					Entity Target = getEntity(centity);
					if (!Target.isDummy())
					{
						continue;
					}
					if(player_glow && !Target.isGlowing())
					{
						Target.enableGlow();
					}
					else if(!player_glow && Target.isGlowing())
					{
						Target.disableGlow();
					}
				

					ProcessPlayer(LPlayer, Target, entitylist, c);
					c++;
				}
			}
			else
			{
				for (int i = 0; i < toRead; i++)
				{
					uint64_t centity = 0;
					apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
					if (centity == 0) continue;
					if (LocalPlayer == centity) continue;

					Entity Target = getEntity(centity);
					if (!Target.isPlayer())
					{
						continue;
					}
					
					ProcessPlayer(LPlayer, Target, entitylist, i);

					int entity_team = Target.getTeamId();
					if (entity_team == team_player)
					{
						continue;
					}
					if(player_glow && !Target.isGlowing())
					{
						Target.enableGlow();
					}
					else if(!player_glow && Target.isGlowing())
					{
						Target.disableGlow();
					}

					
				}
			}

			if(!spectators && !allied_spectators)
			{
				spectators = tmp_spec;
				allied_spectators = tmp_all_spec;
			}
			else
			{
				//refresh spectators count every ~2 seconds
				counter++;
				if(counter==70)
				{
					spectators = tmp_spec;
					allied_spectators = tmp_all_spec;
					counter = 0;
				}
			}

			if(!lock)
				aimentity = tmp_aimentity;
			else
				aimentity = lastaimentity;

			
		}
	}
	actions_t = false;
}

// /////////////////////////////////////////////////////////////////////////////////////////////////////

player players[toRead];

//ESP loop.. this helps right?

//Aimbot Loop stuff
static void AimbotLoop()
{
	aim_t = true;
	while (aim_t)
	{
		std::this_thread::sleep_for(std::chrono::milliseconds(1));
		while (g_Base!=0)
		{
			std::this_thread::sleep_for(std::chrono::milliseconds(1));
			if (aim>0)
			{
				if (aimentity == 0 || !aiming)
				{
					lock=false;
					lastaimentity=0;
					continue;
				}
				lock=true;
				lastaimentity = aimentity;
				uint64_t LocalPlayer = 0;
				apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);
				Entity LPlayer = getEntity(LocalPlayer);
				if (LocalPlayer == 0) continue;
				QAngle Angles = CalculateBestBoneAim(LPlayer, aimentity, max_fov);
				if (Angles.x == 0 && Angles.y == 0)
				{
					lock=false;
					lastaimentity=0;
					continue;
				}
				LPlayer.SetViewAngles(Angles);
			}
		}
	}
	aim_t = false;
}
// Item Glow Stuff
static void item_glow_t()
{
	item_t = true;
	while(item_t)
	{
		std::this_thread::sleep_for(std::chrono::milliseconds(1));
		int k = 0;
		while(g_Base!=0)
		{
			std::this_thread::sleep_for(std::chrono::milliseconds(1));
			uint64_t entitylist = g_Base + OFFSET_ENTITYLIST;
			if (item_glow)
			{
				//item ENTs to loop, 10k-15k is normal. 10k might be better but will not show all the death boxes i think.
				for (int i = 0; i < 15000; i++)
				{
					uint64_t centity = 0;
					apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
					if (centity == 0) continue;
					Item item = getItem(centity);
					//testing
					uint64_t LocalPlayer = 0;
					apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);
						
					if(item.isItem() && !item.isGlowing())
					{
						//item.enableGlow();
					}
					//Item filter glow name setup and search.
					char glowName[200] = { 0 };
					uint64_t name_ptr;
					apex_mem.Read<uint64_t>(centity + OFFSET_MODELNAME, name_ptr);
					apex_mem.ReadArray<char>(name_ptr, glowName, 200);
					
					
					//Level name printf
					//char LevelNAME[200] = { 0 };
					//uint64_t levelname_ptr;
					//apex_mem.Read<uint64_t>(g_Base + OFFSET_LEVELNAME, levelname_ptr);
					//apex_mem.ReadArray<char>(levelname_ptr, LevelNAME, 200);
					
					//printf("%s\n", LevelNAME);
					
					
					//Prints stuff you want to console
					//if (strstr(glowName, "mdl/")) 
					//{
					//printf("%s\n", glowName);
					//}
					//Search model name and if true sets glow, must be a better way to do this.. if only i got the item id to work..
					if (lightbackpack && strstr(glowName, "mdl/humans_r5/loot/w_loot_char_backpack_light.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					
					}
					else 
					{
						apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 0);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 0);
						item.disableGlow();
					}
				
					if (medbackpack && strstr(glowName, "mdl/humans_r5/loot/w_loot_char_backpack_medium.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 2); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 191 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 255 / itemglowbrightness); // b
					
					}
					if (heavybackpack && strstr(glowName, "mdl/humans_r5/loot/w_loot_char_backpack_heavy.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					
					}
					//item id would help so much here, cant make them all the same color so went with loba glow for body shield and helmet
					if (shieldupgrade && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_cha_shield_upgrade_body.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (shieldupgradehead && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_cha_shield_upgrade_head.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (accelerant && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_ultimate_accelerant.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (phoenix && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_phoenix_kit_v1.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 2); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,127 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 255 / itemglowbrightness); // b
					}
					if (item.isBox())
					{
						apex_mem.Write<int>(centity + OFFSET_GLOW_T1, 16256);
						apex_mem.Write<int>(centity + OFFSET_GLOW_T2, 1193322764);
						apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 7);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						
					}
					
					if (item.isTrap())
					{
						apex_mem.Write<int>(centity + OFFSET_GLOW_T1, 16256);
						apex_mem.Write<int>(centity + OFFSET_GLOW_T2, 1193322764);
						apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 7);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						
					}
					
					//Gas Trap
					if (strstr(glowName, "mdl/props/caustic_gas_tank/caustic_gas_tank.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (healthlarge && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_health_main_large.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 128 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (healthsmall && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_health_main_small.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 128 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (shieldbattsmall && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_shield_battery_small.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 225 / itemglowbrightness); // b
					}
					if (shieldbattlarge && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_shield_battery_large.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 225 / itemglowbrightness); // b
					}
					if (ammosniper && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_sniper.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 92 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 92 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 255 / itemglowbrightness); // b
					}
					if (ammohc && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_hc.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 250 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 154 / itemglowbrightness); // b
					}
					if (optic && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_hcog_r1.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (ammosc && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_sc.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 140 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (ammonrg && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_nrg.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 154 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 205 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 50 / itemglowbrightness); // b
					}
					if (ammoshotgun && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_shg.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (lasersight && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_lasersight_v1.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (magsniper && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_sniper_v1.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (magenergy && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_energy_v1.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (stocksniper && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_stock_folded_sniper.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (stockregular && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_stock_folded_regular.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (shielddown && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_shield_down_v1.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (lightammomag && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v1b.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (heavyammomag && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v2b.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (optic2x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_hcog_r2.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (opticholo1x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_holo_var.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (opticholo1x2x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_holo_var_2x.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (opticthreat && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_threat.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (optic3x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_rng_hcog_acgs.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (optic2x4x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_rng_aog_var_r1.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (opticsniper6x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_dcom.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (opticsniper4x8x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_var_talon.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (opticsniperthreat && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_threat_wyeon.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (suppressor && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_suppr_v2b.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (weaponmod && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_chip.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (shotgunbolt && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v3b.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					//Nades
					if (grenade_frag && strstr(glowName, "mdl/weapons/grenades/w_loot_m20_f_grenade_projectile.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					
					if (grenade_thermite && strstr(glowName, "mdl/Weapons/grenades/w_thermite_grenade.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					if (grenade_arc_star && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_shuriken.rmdl")) 
					{
						apex_mem.Write<int>(centity + OFFSET_ITEM_GLOW, 1363184265);
					}
					//Weapons
					if (weapon_kraber && strstr(glowName, "mdl/weapons/at_rifle/w_at_rifle.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0); // b
					}
					if (weapon_mastiff && strstr(glowName, "mdl/weapons/mastiff_stgn/w_mastiff.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_lstar && strstr(glowName, "mdl/weapons/lstar/w_lstar.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 154 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 205 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 50 / itemglowbrightness); // b
					}
					//new gun, nemesis
					if (weapon_nemesis && strstr(glowName, "mdl/techart/mshop/weapons/class/assault/nemesis/nemesis_base_w.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 154 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 205 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 50 / itemglowbrightness); // b
					}
					
					if (weapon_havoc && strstr(glowName, "mdl/Weapons/beam_ar/w_beam_ar.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 154 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 205 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 50 / itemglowbrightness); // b
					}
					if (weapon_devotion && strstr(glowName, "mdl/weapons/hemlock_br/w_hemlock_br.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 154 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 205 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 50 / itemglowbrightness); // b
					}
					if (weapon_triple_take && strstr(glowName, "mdl/weapons/doubletake/w_doubletake.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 154 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 205 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 50 / itemglowbrightness); // b
					}
					if (weapon_flatline  && strstr(glowName, "mdl/techart/mshop/weapons/class/assault/flatline/flatline_base_w.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 250 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 154 / itemglowbrightness); // b
					}
					if (weapon_hemlock && strstr(glowName, "mdl/weapons/m1a1_hemlok/w_hemlok.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 250 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 154 / itemglowbrightness); // b
					}
					if (weapon_g7_scout && strstr(glowName, "mdl/weapons/g2/w_g2a4.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 140 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_alternator && strstr(glowName, "mdl/weapons/alternator_smg/w_alternator_smg.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 140 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_r99 && strstr(glowName, "mdl/weapons/r97/w_r97.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 140 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_prowler && strstr(glowName, "mdl/Weapons/prowler_smg/w_prowler_smg.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 250 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 154 / itemglowbrightness); // b
					}
					if (weapon_volt && strstr(glowName, "mdl/weapons/hemlok_smg/w_hemlok_smg.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 154 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 205 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 50 / itemglowbrightness); // b
					}
					if (weapon_longbow && strstr(glowName, "mdl/weapons/rspn101_dmr/w_rspn101_dmr.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 92 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 92 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 255 / itemglowbrightness); // b
					}
					if (weapon_charge_rifle && strstr(glowName, "mdl/weapons/defender/w_defender.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 92 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 92 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 255 / itemglowbrightness); // b
					}
					if (weapon_spitfire && strstr(glowName, "mdl/weapons/lmg_hemlok/w_lmg_hemlok.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 140 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_r301 && strstr(glowName, "mdl/weapons/rspn101/w_rspn101.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 140 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_eva8 && strstr(glowName, "mdl/weapons/w1128/w_w1128.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_peacekeeper && strstr(glowName, "mdl/weapons/peacekeeper/w_peacekeeper.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_mozambique && strstr(glowName, "mdl/weapons/pstl_sa3/w_pstl_sa3.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_wingman && strstr(glowName, "mdl/weapons/b3wing/w_b3wing.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 92 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 92 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 255 / itemglowbrightness); // b
					}
					if (weapon_p2020 && strstr(glowName, "mdl/weapons/p2011/w_p2011.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 140 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_re45 && strstr(glowName, "mdl/weapons/p2011_auto/w_p2011_auto.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 140 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0 / itemglowbrightness); // b
					}
					if (weapon_sentinel && strstr(glowName, "mdl/Weapons/sentinel/w_sentinel.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 92 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 92 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 255 / itemglowbrightness); // b
					}
					if (weapon_bow && strstr(glowName, "mdl/Weapons/compound_bow/w_compound_bow.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 255); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 0); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 0); // b
					}
					if (weapon_3030_repeater && strstr(glowName, "mdl/weapons/3030repeater/w_3030repeater.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 250 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 154 / itemglowbrightness); // b
					}
					if (weapon_rampage && strstr(glowName, "mdl/techart/mshop/weapons/class/lmg/dragon/dragon_base_w.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 250 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 154 / itemglowbrightness); // b
					}
					if (weapon_car_smg && strstr(glowName, "mdl/techart/mshop/weapons/class/smg/car/car_base_w.rmdl")) 
					{
					apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 1);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 1); // 1 = far, 2 = close
						apex_mem.Write<GlowMode>(centity + GLOW_START_TIME, { 101,101,99,90 });
 
						apex_mem.Write<float>(centity + GLOW_COLOR_R, 0 / itemglowbrightness); // r
						apex_mem.Write<float>(centity + GLOW_COLOR_G, 250 / itemglowbrightness); // g
						apex_mem.Write<float>(centity + GLOW_COLOR_B, 154 / itemglowbrightness); // b
					}
					
					

						
					
				}
				k=1;
				//Change the 60 ms to lower to make the death boxes filker less.
				//std::this_thread::sleep_for(std::chrono::milliseconds(60));
			}
			else
			{		
				if(k==1)
				{
					//same and the ents above to turn the glow off
					for (int i = 0; i < 15000; i++)
					{
						uint64_t centity = 0;
						apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
						if (centity == 0) continue;
 
						Item item = getItem(centity);
 
						if(item.isItem() && item.isGlowing())
						{
							item.disableGlow();
						}
					}
					k=0;
				}
			}	
		}
	}
	item_t = false;
}

auto prevTime = std::chrono::high_resolution_clock::now();
auto currentTime = std::chrono::high_resolution_clock::now();
float deltaTime = 0.0f;

int main(int argc, char *argv[])
{
	currentTime = std::chrono::high_resolution_clock::now();
	deltaTime = std::chrono::duration_cast<std::chrono::milliseconds>(currentTime - prevTime).count() / 1000.0f;
	if(geteuid() != 0)
	{
		//run as root..
		return 0;
	}

	const char* ap_proc = "R5Apex.exe";

	std::thread aimbot_thr;
	std::thread actions_thr;
	std::thread cactions_thr;
	std::thread itemglow_thr;
	while(active)
	{
		if(apex_mem.get_proc_status() != process_status::FOUND_READY)
		{
			if(aim_t)
			{
				aim_t = false;
				actions_t = false;
				cactions_t = false;
				item_t = false;
				g_Base = 0;

				aimbot_thr.~thread();
				actions_thr.~thread();
				cactions_thr.~thread();
				itemglow_thr.~thread();
			}

			std::this_thread::sleep_for(std::chrono::seconds(1));
			printf("Searching for apex process...\n");

			apex_mem.open_proc(ap_proc);

			if(apex_mem.get_proc_status() == process_status::FOUND_READY)
			{
				g_Base = apex_mem.get_proc_baseaddr();
				printf("\nApex process found\n");
				printf("Base: %lx\n", g_Base);

				aimbot_thr = std::thread(AimbotLoop);
				actions_thr = std::thread(DoActions);
				cactions_thr = std::thread(ClientActions);
				itemglow_thr = std::thread(item_glow_t);
				aimbot_thr.detach();
				actions_thr.detach();
				cactions_thr.detach();
				itemglow_thr.detach();
			}
		}
		else
		{
			apex_mem.check_proc();
		}
		std::this_thread::sleep_for(std::chrono::milliseconds(10));
	}
	prevTime = currentTime;
	return 0;
}