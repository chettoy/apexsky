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
#include <array>
#include <map>
//this is a test, with seconds
Memory apex_mem;


//Just setting things up, dont edit.
bool active = true;
uintptr_t aimentity = 0;
uintptr_t tmp_aimentity = 0;
uintptr_t lastaimentity = 0;
float max = 999.0f;
int team_player = 0;
const int toRead = 100;
bool aiming = false;

//Removed but not all the way, dont edit.
int glowtype;
int glowtype2;
float aimdist = 200.0f * 40.0f;
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

//^^ Don't EDIT^^

//CONFIG AREA, you must set all the true/false to what you want.

//Gamepad or Keyboard config, Only one true at once or it wont work.
bool keyboard = true;
bool gamepad = false;
//Done with Gamepad or Keyboard config
bool TDMToggle = false;
bool item_glow = true; //item glow
bool player_glow = true; //player glow
bool aim_no_recoil = true; //no recoil
float max_fov = 15; // Fov you want to use while aiming
int aim = 2; // 0 no aim, 1 aim with no vis check, 2 aim with vis check
bool firing_range = false; //firing range
int bone = 2; //bone 0 head, 1 neck, 2 chest, 3 dick shot
float smooth = 120.0f; //min 85 no beaming, 100 somewhat beam people, 125 should be safe
//Player Glow Color and Brightness.
//inside fill
unsigned char insidevalue = 14;  //0 = no fill, 14 = full fill
//Outline size
unsigned char outlinesize = 200; // 0-255
//Not Visable 
float glowr = 1; //Red 0-255, higher is brighter color.
float glowg = 0; //Green 0-255, higher is brighter color.
float glowb = 0; //Blue 0-255, higher is brighter color.
//Visable
float glowrviz = 0; //Red 0-255, higher is brighter color.
float glowgviz = 1; //Green 0-255, higher is brighter color.
float glowbviz = 0; //Blue 0-255, higher is brighter color.
//Knocked
float glowrknocked = 0.5; //Red 0-255, higher is brighter color.
float glowgknocked = 0.5; //Green 0-255, higher is brighter color.
float glowbknocked = 0.5; //Blue 0-255, higher is brighter color.
//Item Configs
int itemglowbrightness = 8; //10 is none and 0 is full glow like the sun in your eye's.
//rev skull
bool skull = true;
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
bool phoenix = true;
bool healthlarge = true;
bool healthsmall = false;
bool shieldbattsmall = false;
bool shieldbattlarge = true;
//Ammo
bool sniperammo = true;
bool heavyammo = true;
bool lightammo = true;
bool energyammo = true;
bool shotgunammo = false;
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
bool sniperammomag = true;
bool energyammomag = true;
bool lightammomag = true;
bool heavyammomag = true;
//Attachments 
bool lasersight = true;
bool stocksniper = true;
bool stockregular = true;
bool suppressor = true;
bool weaponmod = true;
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
bool weapon_havoc = false;
bool weapon_devotion = false;
bool weapon_triple_take = true;
bool weapon_prowler  = false;
bool weapon_volt  = true;
//Heavy Weapons
bool weapon_flatline = true;
bool weapon_hemlock  = true;
bool weapon_3030_repeater = false; 
bool weapon_rampage  = true;
bool weapon_car_smg  = false;
//Light weapons
bool weapon_p2020  = false;
bool weapon_re45  = true;
bool weapon_g7_scout  = true;
bool weapon_alternator  = false;
bool weapon_r99  = true;
bool weapon_spitfire  = false;
bool weapon_r301 = true;
//Snipers.. wingman is the odd one...and the bow..
bool weapon_wingman  = false;
bool weapon_longbow  = true;
bool weapon_charge_rifle  = false;
bool weapon_sentinel  = false;
bool weapon_bow  = false;
//trigger bot
bool is_trigger;

//DONE WITH THE EDITING
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
struct Matrix
{
	float matrix[16];
};
float lastvis_aim[toRead];
int tmp_spec = 0, spectators = 0;
int tmp_all_spec = 0, allied_spectators = 0;
int glowtype3;
int settingIndex;
int contextId;
std::array<float, 3> highlightParameter;
//works
void SetPlayerGlow(Entity& LPlayer, Entity& Target, int index)
{
	if (player_glow >= 1)
	{
			if (!Target.isGlowing() || (int)Target.buffer[OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE] != 1) {
				float currentEntityTime = 5000.f;
				if (!isnan(currentEntityTime) && currentEntityTime > 0.f) {
					if (!(firing_range) && (Target.isKnocked() || !Target.isAlive()))
					{
						contextId = 5;
						settingIndex = 41;
						highlightParameter = { 0.5, 0.5, 0.5 };
					}
					else if (Target.lastVisTime() > lastvis_aim[index] || (Target.lastVisTime() < 0.f && lastvis_aim[index] > 0.f))
					{
						contextId = 6;
						settingIndex = 42;
						highlightParameter = { 0, 1, 0 };
					}
					else 
					{
						contextId = 7;
						settingIndex = 43;
						highlightParameter = { 1, 0, 0 };
					}

					Target.enableGlow();
				}
			}
		
		else if((player_glow == 0) && Target.isGlowing())
		{
			Target.disableGlow();
		}
		
	}
}

bool IsInCrossHair(Entity& target)
{
    static uintptr_t last_t;
		static float last_crosshair_target_time = -1.f;
		float now_crosshair_target_time = target.lastVisTime();
		bool is_trigger = false;
		if (last_t == target.ptr)
		{
			if (last_crosshair_target_time != -1.f)
			{
				if (now_crosshair_target_time > last_crosshair_target_time)
				{
					is_trigger = true;
					last_crosshair_target_time = -1.f;
				}
				else
				{
					is_trigger = false;
					last_crosshair_target_time = now_crosshair_target_time;
				}
			}
			else
			{
				is_trigger = false;
				last_crosshair_target_time = now_crosshair_target_time;
			}
	 
		}
		else
		{
			last_t = target.ptr;
			last_crosshair_target_time = -1.f;
		}
	 
		return is_trigger;
}


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
			//printf("%f\n", max_fov);
			
			if(keyboard)
			{
				if (attackState == 108 || zoomState == 109)
				{
					aiming = true;
				}
				else
				{
					aiming = false;
				}
				
				
				if (attackState == 108 || !zoomState == 109)
				{
					max_fov = 50;
				}
				if (!attackState == 108 || zoomState == 109)
				{
					max_fov = 15;
				}
			}
			
			if(gamepad)
			{
				if (attackState == 264 || zoomState == 263)
				{
					aiming = true;
				}
				else
				{
					aiming = false;
				}
				
				
				if (attackState == 264 || !zoomState == 263)
				{
					max_fov = 50;
				}
				if (!attackState == 264 || zoomState == 263)
				{
					max_fov = 15;
				}
			}
			
			
			now1 = Clock::now();
			duration1 = std::chrono::duration_cast<std::chrono::milliseconds>(now1 - start1);
			
						
			
			
		
				
			// Toggle crouch = check for ring
			if (attackState != 108 && tduckState == 13)
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

				if (duration >= 500)
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
	
	if (TDMToggle)
	{// Check if the target entity is on the same team as the local player
		//int entity_team = Target.getTeamId();
		//printf("Target Team: %i\n", entity_team);


		uint64_t PlayerLocal;
		apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, PlayerLocal);
		int PlayerLocalTeamID;
		apex_mem.Read<int>(PlayerLocal + OFFSET_TEAM, PlayerLocalTeamID);



		if (entity_team % 2) EntTeam = 1;
		else EntTeam = 2;
		if (PlayerLocalTeamID % 2) LocTeam = 1;
		else LocTeam = 2;

		//printf("Target Team: %i\nLocal Team: %i\n", EntTeam, LocTeam);
		if (EntTeam == LocTeam)
			return;

	}
	

	Vector EntityPosition = target.getPosition();
	Vector LocalPlayerPosition = LPlayer.getPosition();
	float dist = LocalPlayerPosition.DistTo(EntityPosition);
	//if (dist > aimdist) return;
	
	
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
	SetPlayerGlow(LPlayer, target, index);
	lastvis_aim[index] = target.lastVisTime();
}
std::map<uint64_t, int> centityToNumber; // Map centity to a unique number
int uniqueNumber = 1; // Initialize a unique number
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
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 72;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					
					}
					/* else 
					{
						apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, 0);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 0);
						item.disableGlow();
					} */
				
					if (medbackpack && strstr(glowName, "mdl/humans_r5/loot/w_loot_char_backpack_medium.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 69;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);	
					
					}
					if (heavybackpack && strstr(glowName, "mdl/humans_r5/loot/w_loot_char_backpack_heavy.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.6275, 0.1255, 0.9412 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);	
					
					}
					//item id would help so much here, cant make them all the same color so went with loba glow for body shield and helmet
					if (shieldupgrade && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_cha_shield_upgrade_body.rmdl")) 
					{
						
					}
					if (shieldupgradehead && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_cha_shield_upgrade_head.rmdl")) 
					{
						
					}
					if (accelerant && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_ultimate_accelerant.rmdl")) 
					{
						
					}
					if (phoenix && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_phoenix_kit_v1.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.6275, 0.1255, 0.9412 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (skull && strstr(glowName, "mdl/Weapons/skull_grenade/skull_grenade_base_v.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.6275, 0.1255, 0.9412 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (item.isBox())
					{
						
						
					}
					
					if (item.isTrap())
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);	
						
					}
					
					//Gas Trap
					if (strstr(glowName, "mdl/props/caustic_gas_tank/caustic_gas_tank.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (healthlarge && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_health_main_large.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 72;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (healthsmall && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_health_main_small.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 72;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shieldbattsmall && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_shield_battery_small.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 69;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shieldbattlarge && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_shield_battery_large.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 69;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (sniperammo && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_sniper.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 69;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (heavyammo && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_hc.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 65;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (optic && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_hcog_r1.rmdl")) 
					{
						
					}
					if (lightammo && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_sc.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.5490, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 66;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (energyammo && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_nrg.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2, 1, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 73;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shotgunammo && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_ammo_shg.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (lasersight && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_lasersight_v1.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							137,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							138,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						//std::array<float, 3> highlightParameter = { 0, 1, 1 };
						//OFFSET_HIGHLIGHTSETTINGS = 0xb5f9620;
						//OFFSET_HIGHLIGHTSERVERACTIVESTATES = 0x298;
						//OFFSET_HIGHLIGHTCURRENTCONTEXTID = 0x294;
						//OFFSET_HIGHLIGHTVISIBILITYTYPE = 0x278;
						apex_mem.Write<uint32_t>(centity + 0x278, 2);
						uint32_t contextId;
						apex_mem.Read<uint32_t>(centity + 0x294, contextId);
						uint8_t hightState;
						apex_mem.Read<uint8_t>(centity + contextId + 0x298, hightState);
						uint64_t HighlightSettings;
						apex_mem.Read<uint64_t>(g_Base + 0xb5f9620, HighlightSettings);
						
						apex_mem.Write<typeof(highlightFunctionBits)>(HighlightSettings + 40 * hightState + 4, highlightFunctionBits); 
						//apex_mem.Write<typeof(highlightParameter)>(HighlightSettings + 40 * hightState + 8, highlightParameter);
					}
					if (sniperammomag && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_sniper_v1.rmdl")) 
					{
						
					}
					if (energyammomag && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_energy_v1.rmdl")) 
					{
						
					}
					if (stocksniper && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_stock_folded_sniper.rmdl")) 
					{
					
					}
					if (stockregular && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_stock_folded_regular.rmdl")) 
					{
						
					}
					if (shielddown && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_shield_down_v1.rmdl")) 
					{
						
					}
					if (lightammomag && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v1b.rmdl")) 
					{
						
					}
					if (heavyammomag && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v2b.rmdl")) 
					{
						
					}
					if (optic2x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_hcog_r2.rmdl")) 
					{
						
					}
					if (opticholo1x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_holo_var.rmdl")) 
					{
						
					}
					if (opticholo1x2x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_holo_var_2x.rmdl")) 
					{
						
					}
					if (opticthreat && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_cq_threat.rmdl")) 
					{
						
					}
					if (optic3x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_rng_hcog_acgs.rmdl")) 
					{
						
					}
					if (optic2x4x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_rng_aog_var_r1.rmdl")) 
					{
						
					}
					if (opticsniper6x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_dcom.rmdl")) 
					{
						
					}
					if (opticsniper4x8x && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_var_talon.rmdl")) 
					{
						
					}
					if (opticsniperthreat && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_optic_sni_threat_wyeon.rmdl")) 
					{
						
					}
					if (suppressor && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_suppr_v2b.rmdl")) 
					{
						
					}
					if (weaponmod && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_chip.rmdl")) 
					{
						
					}
					if (shotgunbolt && strstr(glowName, "mdl/weapons_r5/loot/_master/w_loot_wep_mods_mag_v3b.rmdl")) 
					{
						
					}
					//Nades
					if (grenade_frag && strstr(glowName, "mdl/weapons/grenades/w_loot_m20_f_grenade_projectile.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);	
					}
					
					if (grenade_thermite && strstr(glowName, "mdl/Weapons/grenades/w_thermite_grenade.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);	
					}
					if (grenade_arc_star && strstr(glowName, "mdl/weapons_r5/loot/w_loot_wep_iso_shuriken.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 70;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);	
					}
					//Weapons
					if (weapon_kraber && strstr(glowName, "mdl/weapons/at_rifle/w_at_rifle.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_mastiff && strstr(glowName, "mdl/techart/mshop/weapons/class/shotgun/mastiff/mastiff_base_w.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_lstar && strstr(glowName, "mdl/weapons/lstar/w_lstar.rmdl")) 
					{
					
					}
					//new gun, nemesis
					if (weapon_nemesis && strstr(glowName, "mdl/techart/mshop/weapons/class/assault/nemesis/nemesis_base_w.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2, 1, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 73;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					
					if (weapon_havoc && strstr(glowName, "mdl/Weapons/beam_ar/w_beam_ar.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2, 1, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 73;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_devotion && strstr(glowName, "mdl/weapons/hemlock_br/w_hemlock_br.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2, 1, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 73;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_triple_take && strstr(glowName, "mdl/weapons/doubletake/w_doubletake.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2, 1, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 73;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_flatline  && strstr(glowName, "mdl/techart/mshop/weapons/class/assault/flatline/flatline_base_w.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 65;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_hemlock && strstr(glowName, "mdl/weapons/m1a1_hemlok/w_hemlok.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 65;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_g7_scout && strstr(glowName, "mdl/techart/mshop/weapons/class/assault/g7/g7_base_w.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.5490, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 66;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_alternator && strstr(glowName, "mdl/weapons/alternator_smg/w_alternator_smg.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.5490, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 66;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_r99 && strstr(glowName, "mdl/weapons/r97/w_r97.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.5490, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 66;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_prowler && strstr(glowName, "mdl/Weapons/prowler_smg/w_prowler_smg.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 65;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_volt && strstr(glowName, "mdl/weapons/hemlok_smg/w_hemlok_smg.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2, 1, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 73;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_longbow && strstr(glowName, "mdl/weapons/rspn101_dmr/w_rspn101_dmr.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 69;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_charge_rifle && strstr(glowName, "mdl/weapons/defender/w_defender.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 69;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_spitfire && strstr(glowName, "mdl/weapons/lmg_hemlok/w_lmg_hemlok.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.5490, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 66;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_r301 && strstr(glowName, "mdl/weapons/rspn101/w_rspn101.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.5490, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 66;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_eva8 && strstr(glowName, "mdl/weapons/w1128/w_w1128.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_peacekeeper && strstr(glowName, "mdl/weapons/peacekeeper/w_peacekeeper.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_mozambique && strstr(glowName, "mdl/weapons/pstl_sa3/w_pstl_sa3.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_wingman && strstr(glowName, "mdl/weapons/b3wing/w_b3wing.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 69;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_p2020 && strstr(glowName, "mdl/weapons/p2011/w_p2011.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.5490, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 66;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_re45 && strstr(glowName, "mdl/weapons/p2011_auto/w_p2011_auto.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.5490, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 66;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_sentinel && strstr(glowName, "mdl/Weapons/sentinel/w_sentinel.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 0, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 69;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_bow && strstr(glowName, "mdl/Weapons/compound_bow/w_compound_bow.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 67;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_3030_repeater && strstr(glowName, "mdl/weapons/3030repeater/w_3030repeater.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 65;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (weapon_rampage && strstr(glowName, "mdl/techart/mshop/weapons/class/lmg/dragon/dragon_base_w.rmdl")) 
					{
					
						//works
						/* std::array<unsigned char, 4> highlightFunctionBits = {
							137,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							138,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						//OFFSET_HIGHLIGHTSETTINGS = 0xb5f9620;
						//OFFSET_HIGHLIGHTSERVERACTIVESTATES = 0x298;
						//OFFSET_HIGHLIGHTCURRENTCONTEXTID = 0x294;
						//OFFSET_HIGHLIGHTVISIBILITYTYPE = 0x278;
						apex_mem.Write<uint32_t>(centity + 0x278, 2);
						uint32_t contextId;
						apex_mem.Read<uint32_t>(centity + 0x294, contextId);
						uint8_t hightState;
						apex_mem.Read<uint8_t>(centity + contextId + 0x298, hightState);
						uint64_t HighlightSettings;
						apex_mem.Read<uint64_t>(g_Base + 0xb5f9620, HighlightSettings);
						
						apex_mem.Write<typeof(highlightFunctionBits)>(HighlightSettings + 40 * hightState + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(HighlightSettings + 40 * hightState + 8, highlightParameter);  */
						
						
						//Works better
						/* std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						//OFFSET_HIGHLIGHTSETTINGS = 0xb5f9620;
						//OFFSET_HIGHLIGHTSERVERACTIVESTATES = 0x298;
						//OFFSET_HIGHLIGHTCURRENTCONTEXTID = 0x294;
						//OFFSET_HIGHLIGHTVISIBILITYTYPE = 0x278;
						apex_mem.Write<uint32_t>(centity + 0x278, 2);
						static const int contextId = 0;
						int settingIndex = 65;
						//apex_mem.Read<uint32_t>(centity + 0x294, contextId);
						//printf("%d\n", contextId);
						//uint8_t hightState;
						//apex_mem.Read<uint8_t>(centity + contextId + 0x298, hightState);
						//uint64_t HighlightSettings;
						//apex_mem.Read<uint64_t>(g_Base + 0xb5f9620, HighlightSettings);
						
						apex_mem.Write<unsigned char>(centity + 0x298 + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + 0xb5f9620, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter); */ 
						
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						//OFFSET_HIGHLIGHTSETTINGS = 0xb5f9620;
						//OFFSET_HIGHLIGHTSERVERACTIVESTATES = 0x298;
						//OFFSET_HIGHLIGHTCURRENTCONTEXTID = 0x294;
						//OFFSET_HIGHLIGHTVISIBILITYTYPE = 0x278;
						apex_mem.Write<uint32_t>(centity + 0x278, 2);
						static const int contextId = 0;
						int settingIndex = 65;
						//apex_mem.Read<uint32_t>(centity + 0x294, contextId);
						//printf("%d\n", contextId);
						//uint8_t hightState;
						//apex_mem.Read<uint8_t>(centity + contextId + 0x298, hightState);
						//uint64_t HighlightSettings;
						//apex_mem.Read<uint64_t>(g_Base + 0xb5f9620, HighlightSettings);
						
						apex_mem.Write<unsigned char>(centity + 0x298 + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + 0xb5f9620, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
						
						
						
					}
					if (weapon_car_smg && strstr(glowName, "mdl/techart/mshop/weapons/class/smg/car/car_base_w.rmdl")) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							125,
							64
						};
						std::array<float, 3> highlightParameter = { 0, 1, 1 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 65;
												apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					} 

					// CREDITS to Rikkie https://www.unknowncheats.me/forum/members/169606.html
					// for all the weapon ids and item ids code, you are a life saver!
					ulong ehWeaponHandle;
					apex_mem.Read<uint64_t>(LocalPlayer + OFFSET_WEAPON, ehWeaponHandle); // 0x1a1c
					ehWeaponHandle &= 0xFFFF; // eHandle
					ulong pWeapon;
					apex_mem.Read<uint64_t>(entitylist + (ehWeaponHandle * 0x20), pWeapon);
					
					uint32_t weaponID;
					apex_mem.Read<uint32_t>(pWeapon + OFFSET_WEAPON_NAME, weaponID); //0x1844
					//printf("%d\n", weaponID);
					//snipers for headsbots
					if (weaponID == 101 || weaponID == 87 || weaponID == 2 || weaponID == 84 || weaponID == 1 || weaponID == 78 || weaponID == 80 || weaponID == 102 || weaponID == 104 || weaponID == 105)
					{
					
						bone = 0;
					}
					else if (weaponID != 101 || weaponID != 87 || weaponID != 2 || weaponID != 84 || weaponID != 1 || weaponID != 78 || weaponID != 80 || weaponID != 102 || weaponID != 104 || !weaponID == 105)
					{
						bone = 2;
					}
					//bow
			
					
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




int main(int argc, char *argv[])
{
	
	if(geteuid() != 0)
	{
		//run as root..
		return 0;
	}

	const char* ap_proc = "r5apex.exe";

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
	
	return 0;
}
