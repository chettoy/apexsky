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
#include <cstdlib> // For the system() function
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
bool updateInsideValue_t = false;
bool terminal_t = false;
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
//Used to change things on a timer
/* unsigned char insidevalueItem = 1;
void updateInsideValue()
{
	updateInsideValue_t = true;
	while (updateInsideValue_t)
	{
		insidevalueItem++;
		insidevalueItem %= 256;
		std::this_thread::sleep_for(std::chrono::milliseconds(300));
		
		printf("insidevalueItem: %i\n", insidevalueItem);
		
	}
	updateInsideValue_t = false;
} */
//Gamepad or Keyboard config, Only one true at once or it wont work.
bool keyboard = true;
bool gamepad = false;
//Done with Gamepad or Keyboard config
//Terminal Stuff
bool lootfilledtoggle = true;
bool playerfilledtoggle = true;
//end Terminal Stuff
bool TDMToggle = false;
bool item_glow = true; //item glow
bool player_glow = true; //player glow
bool aim_no_recoil = true; //no recoil
float max_fov = 15; // Fov you want to use while aiming
int aim = 2; // 0 no aim, 1 aim with no vis check, 2 aim with vis check
//aimbot for nades on or off
bool NoNadeAim = true;
bool firing_range = false; //firing range
int bone = 2; //bone 0 head, 1 neck, 2 chest, 3 dick shot
float smooth = 120.0f; //min 85 no beaming, 100 somewhat beam people, 125 should be safe
//Player Glow Color and Brightness.
//inside fill
unsigned char insidevalue = 14;  //0 = no fill, 14 = full fill
//Outline size
unsigned char outlinesize = 32; // 0-255
//Not Visable 
float glowrnot = 1; //Red 0-1, higher is brighter color.
float glowgnot = 0; //Green 0-1, higher is brighter color.
float glowbnot = 0; //Blue 0-1, higher is brighter color.
//Visable
float glowrviz = 0; //Red 0-1, higher is brighter color.
float glowgviz = 1; //Green 0-1, higher is brighter color.
float glowbviz = 0; //Blue 0-1, higher is brighter color.
//Knocked
float glowrknocked = 0.5; //Red 0-1, higher is brighter color.
float glowgknocked = 0.5; //Green 0-1, higher is brighter color.
float glowbknocked = 0.5; //Blue 0-1, higher is brighter color.
//Item Configs
//loot Fill
unsigned char lootfilled = 14;  //0 no fill, 14 100% fill
//rev skull
bool skull = true;
//Backpacks
bool lightbackpack = false;
bool medbackpack = true;
bool heavybackpack = true;
bool goldbackpack = true;
//Shield upgrades
bool shieldupgrade1 = false;  //white
bool shieldupgrade2 = true;  //blue
bool shieldupgrade3 = true;  //purple
bool shieldupgrade4 = true;  //gold
bool shieldupgrade5 = true;  //red
bool shieldupgradehead1 = false;
bool shieldupgradehead2 = true;
bool shieldupgradehead3 = true;
bool shieldupgradehead4 = true;
bool shielddown1 = false;
bool shielddown2 = true;
bool shielddown3 = true;
bool shielddown4 = true;
//heaing and Misc
bool accelerant = false;
bool phoenix = true;
bool healthlarge = true;
bool healthsmall = false;
bool shieldbattsmall = false;
bool shieldbattlarge = true;
//Ammo
bool sniperammo = false;
bool heavyammo = true;
bool lightammo = true;
bool energyammo = true;
bool shotgunammo = false;
//Optics
bool optic1xhcog = false;
bool optic2xhcog = true;
bool opticholo1x = false;
bool opticholo1x2x = true;
bool opticthreat = false;
bool optic3xhcog = true;
bool optic2x4x = true;
bool opticsniper6x = false;
bool opticsniper4x8x = true;
bool opticsniperthreat = false;
//Magazines
bool sniperammomag1 = false;
bool energyammomag1 = true;
bool lightammomag1 = true;
bool heavyammomag1 = true;
bool sniperammomag2 = false;
bool energyammomag2 = true;
bool lightammomag2 = true;
bool heavyammomag2 = true;
bool sniperammomag3 = false;
bool energyammomag3 = true;
bool lightammomag3 = true;
bool heavyammomag3 = true;
bool sniperammomag4 = false;
bool energyammomag4 = true;
bool lightammomag4 = true;
bool heavyammomag4 = true;
//Attachments 
bool lasersight1 = false;
bool lasersight2 = true;
bool lasersight3 = true;
bool lasersight4 = true;
bool stocksniper1 = false;
bool stocksniper2 = true;
bool stocksniper3 = true;
bool stocksniper4 = true;
bool stockregular1 = false;
bool stockregular2 = true;
bool stockregular3 = true;
bool suppressor1 = false;
bool suppressor2 = true;
bool suppressor3 = true;
bool turbo_charger = false;
bool skull_piecer = false;
bool hammer_point = true;
bool disruptor_rounds = true;
bool boosted_loader = false;
bool shotgunbolt1 = false;
bool shotgunbolt2 = false;
bool shotgunbolt3 = false;
bool shotgunbolt4 = false;
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
bool weapon_triple_take = false;
bool weapon_prowler  = false;
bool weapon_volt  = true;
//Heavy Weapons
bool weapon_flatline = true;
bool weapon_hemlock  = true;
bool weapon_3030_repeater = false; 
bool weapon_rampage  = false;
bool weapon_car_smg  = true;
//Light weapons
bool weapon_p2020  = false;
bool weapon_re45  = true;
bool weapon_g7_scout  = false;
bool weapon_alternator  = false;
bool weapon_r99  = true;
bool weapon_spitfire  = true;
bool weapon_r301 = true;
//Snipers.. wingman is the odd one...and the bow..
bool weapon_wingman  = false;
bool weapon_longbow  = false;
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
						settingIndex = 80;
						highlightParameter = { glowrknocked, glowgknocked, glowbknocked };
					}
					else if (Target.lastVisTime() > lastvis_aim[index] || (Target.lastVisTime() < 0.f && lastvis_aim[index] > 0.f))
					{
						contextId = 6;
						settingIndex = 81;
						highlightParameter = { glowrviz, glowgviz, glowbviz };
					}
					else 
					{
						contextId = 7;
						settingIndex = 82;
						highlightParameter = { glowrnot, glowgnot, glowbnot };
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
		
		/* driver.Writevirtual<uint32_t>(driver.GameBase + OFFSET_IN_ATTACK + 0x8, 4);
		Sleep(1);
		driver.Writevirtual<uint32_t>(driver.GameBase + OFFSET_IN_ATTACK + 0x8, 5);
		Sleep(1);
		driver.Writevirtual<uint32_t>(driver.GameBase + OFFSET_IN_ATTACK + 0x8, 4); */
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
					max_fov = 3;
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
					max_fov = 3;
				}
			}
			
			
			now1 = Clock::now();
			duration1 = std::chrono::duration_cast<std::chrono::milliseconds>(now1 - start1);
			
						
			
			
		
				
			// Toggle crouch = check for ring
			if (attackState != 108 && tduckState == 65)
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
					
					//item ids?
					uint64_t ItemID;
					apex_mem.Read<uint64_t>(centity + OFFSET_ITEM_ID, ItemID);
					/* uint64_t ItemID2;
					ItemID2 = ItemID % 301;
					printf("%ld\n", ItemID2); */
					
					//Level name printf
					//char LevelNAME[200] = { 0 };
					//uint64_t levelname_ptr;
					//apex_mem.Read<uint64_t>(g_Base + OFFSET_LEVELNAME, levelname_ptr);
					//apex_mem.ReadArray<char>(levelname_ptr, LevelNAME, 200);
					
					//printf("%s\n", LevelNAME);
					
					
					//Prints stuff you want to console
					//if (strstr(glowName, "mdl/")) 
					//{
					//printf("%ld\n", ItemID);
					//}
					//Search model name and if true sets glow, must be a better way to do this.. if only i got the item id to work..
					if (lightbackpack && ItemID == 206) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (medbackpack && ItemID == 207) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (heavybackpack && ItemID == 208) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);	
					
					}
					if (goldbackpack && ItemID == 209) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);	
					
					}
					//item id would help so much here, cant make them all the same color so went with loba glow for body shield and helmet
					if (shieldupgrade1 && (ItemID == 214748364992 || ItemID == 14073963583897797))
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
						
						uint64_t ItemID;
						apex_mem.Read<uint64_t>(centity + OFFSET_ITEM_ID, ItemID);
						//uint64_t ItemID2;
						//ItemID2 = ItemID % 301;
						//printf("%ld\n", ItemID);
						//apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);;
					}
					if (shieldupgrade2 && (ItemID == 322122547393 || ItemID == 21110945375846598))
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shieldupgrade3 && (ItemID == 429496729794 || ItemID == 52776987629977799)) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shieldupgrade4 && (ItemID == 429496729795 || ItemID == 536870912200))  
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shieldupgrade5 && ItemID == 536870912200)  
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shieldupgradehead1 && ItemID == 187) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shieldupgradehead2 && ItemID == 188) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shieldupgradehead3 && ItemID == 189) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shieldupgradehead4 && ItemID == 190) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (accelerant && ItemID == 181) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (phoenix && ItemID == 182) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
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
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (item.isBox())
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 88;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
						
					}
					
					if (item.isTrap())
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
							64,
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
					if (healthlarge && ItemID == 183) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (healthsmall && ItemID == 184) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shieldbattsmall && ItemID == 186) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shieldbattlarge && ItemID == 185) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (sniperammo && ItemID == 143) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (heavyammo && ItemID == 142) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (optic1xhcog && ItemID == 214) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (lightammo && ItemID == 139) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (energyammo && ItemID == 140) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shotgunammo && ItemID == 141) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (lasersight1 && ItemID == 228) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (lasersight2 && ItemID == 229) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (lasersight3 && ItemID == 230) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (sniperammomag1 && ItemID == 243) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (sniperammomag2 && ItemID == 244) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (sniperammomag3 && ItemID == 245) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (sniperammomag4 && ItemID == 246) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (energyammomag1 && ItemID == 239) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (energyammomag2 && ItemID == 240) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (energyammomag3 && ItemID == 241) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (energyammomag4 && ItemID == 242) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (stocksniper1 && ItemID == 254) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
								lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
								125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
								64,
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
					if (stocksniper2 && ItemID == 255) 
					{
					std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (stocksniper3 && ItemID == 256) 
					{
					std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (stockregular1 && ItemID == 251) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (stockregular2 && ItemID == 252) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (stockregular3 && ItemID == 253) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shielddown1 && ItemID == 202) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shielddown2 && ItemID == 203) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shielddown3 && ItemID == 204) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shielddown4 && ItemID == 205) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (lightammomag1 && ItemID == 231) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (lightammomag2 && ItemID == 232) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (lightammomag3 && ItemID == 233) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (lightammomag4 && ItemID == 234) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (heavyammomag1 && ItemID == 235) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (heavyammomag2 && ItemID == 236) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (heavyammomag3 && ItemID == 237) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (heavyammomag4 && ItemID == 238) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (optic2xhcog && ItemID == 215) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (opticholo1x && ItemID == 216) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (opticholo1x2x && ItemID == 217) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (opticthreat && ItemID == 218) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (optic3xhcog && ItemID == 219) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (optic2x4x && ItemID == 220) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (opticsniper6x && ItemID == 221) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (opticsniper4x8x && ItemID == 222) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (opticsniperthreat && ItemID == 223) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (suppressor1 && ItemID == 224) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (suppressor2 && ItemID == 225) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (suppressor3 && ItemID == 226) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (turbo_charger && ItemID == 257) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (skull_piecer && ItemID == 259) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (hammer_point && ItemID == 261) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (disruptor_rounds && ItemID == 260) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (boosted_loader && ItemID == 271) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shotgunbolt1 && ItemID == 247) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shotgunbolt2 && ItemID == 248) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (shotgunbolt3 && ItemID == 249) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 0.2941, 0, 0.5098 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 74;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					if (shotgunbolt4 && ItemID == 250) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
							64
						};
						std::array<float, 3> highlightParameter = { 1, 0.8431, 0 };
						apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
						static const int contextId = 0;
						int settingIndex = 75;
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 40 * settingIndex + 4, highlightFunctionBits); 
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 40 * settingIndex + 8, highlightParameter);
					}
					//Nades
					if (grenade_frag && ItemID == 212) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					
					if (grenade_thermite && ItemID == 211) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (grenade_arc_star && ItemID == 213) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_kraber && ItemID == 1) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_mastiff && ItemID == 2) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							14,   // InsideFunction
							125, // OutlineFunction: HIGHLIGHT_OUTLINE_OBJECTIVE
							64,  // OutlineRadius: size * 255 / 8
							64   // (EntityVisible << 6) | State & 0x3F | (AfterPostProcess << 7)
						};
						std::array<float, 3> highlightParameter = { 1, 0, 0 };
						int settingIndex = 67;
						static const int contextId = 2;
						apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, contextId);
						apex_mem.Write<unsigned char>(centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						//apex_mem.Write<int>(ptr + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex);
						long highlightSettingsPtr;
						apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
						apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS , 2);
						apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr + 0x28 * settingIndex + 4, highlightFunctionBits);
						apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr + 0x28 * settingIndex + 8, highlightParameter);
					}
					if (weapon_lstar && ItemID == 8) 
					{
					std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					//new gun, nemesis
					if (weapon_nemesis && ItemID == 134) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					
					if (weapon_havoc && ItemID == 13) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_devotion && ItemID == 18) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_triple_take && ItemID == 23) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_flatline  && ItemID == 28) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_hemlock && ItemID == 33) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_g7_scout && ItemID == 39) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_alternator && ItemID == 44) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_r99 && ItemID == 49) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_prowler && ItemID == 55) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_volt && ItemID == 60) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_longbow && ItemID == 65) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_charge_rifle && ItemID == 70) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
						uint64_t ItemID;
						apex_mem.Read<uint64_t>(centity + OFFSET_ITEM_ID, ItemID);
						//printf("%ld\n", ItemID);
					}
					if (weapon_spitfire && ItemID == 75) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_r301 && ItemID == 80) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_eva8 && ItemID == 85) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_peacekeeper && ItemID == 90) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_mozambique && ItemID == 95) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_wingman && ItemID == 105) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_p2020 && ItemID == 110) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_re45 && ItemID == 115) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_sentinel && ItemID == 121) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_bow && ItemID == 126) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_3030_repeater && ItemID == 128) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_rampage && ItemID == 145) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					if (weapon_car_smg && ItemID == 150) 
					{
						std::array<unsigned char, 4> highlightFunctionBits = {
							lootfilled,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
							125,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED 
							64,
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
					
					
					enum HeldWeaponID
					{
						r301 = 0,
						sentinel = 1,
						bow = 2,
						rampage = 19,
						alternator = 75,
						re45,
						chargerifle,
						devotion,
						longbow,
						havoc,
						eva8,
						flatline,
						scout,
						hemlock,
						kraber,
						lstar,
						mastiff,
						mozambique,
						prowler,
						peacekeeper,
						r99,
						p2020,
						spitfire,
						tripletake,
						wingman,
						volt,
						repeater,
						car,
						nemesis,
						melee_survival = 100,
						shield,
						heirloom_wraith = 130, 
						heirloom_valkyrie = 136, 
						throwing_knife = 139
					};
					
					//Nade test
					int HeldID;
					apex_mem.Read<int>(LocalPlayer + OFFSET_OFF_WEAPON, HeldID); // 0x1a1c
					
					if(NoNadeAim)
					{
						if (HeldID == -251)
						{
							aim = 0;
						}
						else
						{
							aim = 2;
						}
					}
					uint32_t weaponID;
					apex_mem.Read<uint32_t>(pWeapon + OFFSET_MODELNAME, weaponID); //0x1844
					//printf("%d\n", HeldID);
					//snipers for headsbots
					/* if (weaponID == 101 || weaponID == 87 || weaponID == 2 || weaponID == 84 || weaponID == 1 || weaponID == 78 || weaponID == 80 || weaponID == 102 || weaponID == 104 || weaponID == 105)
					{
					
						bone = 0;
					}
					else if (weaponID != 101 || weaponID != 87 || weaponID != 2 || weaponID != 84 || weaponID != 1 || weaponID != 78 || weaponID != 80 || weaponID != 102 || weaponID != 104 || !weaponID == 105)
					{
						bone = 2;
					} */
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

//SSH terminal
const char* boneDescriptions[] = {
    "Head",
    "Neck",
    "Chest",
    "Gut Shot"
};
void updateGlowColor(float &glowr, float &glowg, float &glowb, const std::string &setName)
{
    std::cout << "Enter RGB values for " << setName << " (0-1 for each channel):\n";
    std::cout << "Red: ";
    std::cin >> glowr;

    std::cout << "Green: ";
    std::cin >> glowg;

    std::cout << "Blue: ";
    std::cin >> glowb;

    // Validate and clamp values to the range [0, 1].
    glowr = std::max(0.0f, std::min(1.0f, glowr));
    glowg = std::max(0.0f, std::min(1.0f, glowg));
    glowb = std::max(0.0f, std::min(1.0f, glowb));

    std::cout << setName << " RGB values updated (R: " << glowr << ", G: " << glowg << ", B: " << glowb << ").\n";
}

void terminal()
{
	
	while (true)
	{
		
		system("clear"); // Use "cls" for Windows
		std::string userInput;
		std::cout << "Available commands:" << std::endl;
        
        if (firing_range)
        {
            std::cout << "1 - Firing Range Enabled" << std::endl;
        }
        else
        {
            std::cout << "1 - Firing Range Disabled" << std::endl;
        }
		if (TDMToggle)
        {
            std::cout << "2 - TDMToggle Enabled" << std::endl;
        }
        else
        {
            std::cout << "2 - TDMToggle Disabled" << std::endl;
        }
		if (keyboard)
        {
            std::cout << "3 - Keyboard Enabled" << std::endl;
        }
        else
        {
            std::cout << "3 - Keyboard Disabled" << std::endl;
        }
		if (gamepad)
        {
            std::cout << "4 - Gamepad Enabled" << std::endl;
        }
        else
        {
            std::cout << "4 - Gamepad Disabled" << std::endl;
        }
		if (item_glow)
        {
            std::cout << "5 - Item Glow Enabled" << std::endl;
        }
        else
        {
            std::cout << "5 - Item Glow Disabled" << std::endl;
        }
		if (player_glow)
        {
            std::cout << "6 - Player Glow Enabled" << std::endl;
        }
        else
        {
            std::cout << "6 - Player Glow Disabled" << std::endl;
        }
		
		std::cout << "7 - Change Smooth Value: (Current: ";
        if (smooth < 100.0f)
        {
            std::cout << "\033[1;31m"; // Set text color to red for values below 100
        }
        else if (smooth > 120.0f)
        {
            std::cout << "\033[1;32m"; // Set text color to green for values above 120
        }
        std::cout << smooth << "\033[0m"; // Reset text color to default and close color tag
        std::cout << ")" << std::endl;
		
		std::cout << "8 - Change Bone Aim Value: (Current: ";
		if (bone == 0)
		{
			std::cout << "Head";
		}
		else if (bone == 1)
		{
			std::cout << "Neck";
		}
		else if (bone == 2)
		{
			std::cout << "Chest";
		}
		else if (bone == 3)
		{
			std::cout << "Gut Shot";
		}
		else
		{
			std::cout << "Unknown";
		}
		std::cout << ")" << std::endl;
		
		if (lootfilledtoggle)
        {
			lootfilled = 14;
            std::cout << "9 - Loot Glow Filled" << std::endl;
        }
        else
        {
			lootfilled = 0;
            std::cout << "9 - Loot Glow Not Filled" << std::endl;
        }
		if (playerfilledtoggle)
        {
			insidevalue = 14;
            std::cout << "10 - Player Glow Filled" << std::endl;
        }
        else
        {
			insidevalue = 0;
            std::cout << "10 - Player Glow Not Filled" << std::endl;
        }
		std::cout << "11 - Player Outline Glow" << std::endl;
		std::cout << "12 - Update Glow Colors\n";
		
		std::cout << "Enter a command: ";
        std::getline(std::cin, userInput);
        
        if (userInput == "1")
        {
            // Toggle the firing_range.
            firing_range = !firing_range;
            
            if (firing_range)
            {
                std::cout << "Firing Range ON.\n";
            }
            else
            {
                std::cout << "Firing Range OFF.\n";
            }
        }
		
		if (userInput == "2")
        {
            // Toggle TDM.
            TDMToggle = !TDMToggle;
            
            if (TDMToggle)
            {
                std::cout << "TDM ON.\n";
            }
            else
            {
                std::cout << "TDM OFF.\n";
            }
        }
		if (userInput == "3")
        {
            // Keyboard Enable.
            keyboard = true;
			gamepad = false;
            std::cout << "Keyboard ON.\n";
            
        }
		if (userInput == "4")
        {
            // Gamepad Enable.
            keyboard = false;
			gamepad = true;
            std::cout << "Gamepad ON.\n";
            
        }
		if (userInput == "5")
        {
            // Toggle TDM.
            item_glow = !item_glow;
            
            if (item_glow)
            {
                std::cout << "Item Glow ON.\n";
            }
            else
            {
                std::cout << "Item Glow OFF.\n";
            }
        }
		if (userInput == "6")
        {
            // Toggle TDM.
            player_glow = !player_glow;
            
            if (player_glow)
            {
                std::cout << "Player Glow ON.\n";
            }
            else
            {
                std::cout << "Player Glow OFF.\n";
            }
        }
		if (userInput == "7")
        {
            // Command to change the 'smooth' value.
            std::cout << "Enter a new value for 'smooth' (85 to 200): ";
            float newSmooth;
            std::cin >> newSmooth;

            // Check if the new value is within the desired range.
            if (newSmooth >= 85.0f && newSmooth <= 200.0f)
            {
                smooth = newSmooth;
                std::cout << "'smooth' value updated to: " << smooth << std::endl;
				printf("The value of 'smooth' is: %f\n", smooth);
            }
            else
            {
                std::cout << "Invalid value. 'smooth' value must be between 85 and 200." << std::endl;
            }
            
            // Clear the input buffer to prevent any issues with future input.
            std::cin.clear();
            std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        }
		if (userInput == "8")
        {
            // Command to change the 'smooth' value.
            std::cout << "Enter a new value for 'bone' (0 to 3): ";
            int newBone;
            std::cin >> newBone;

            // Check if the new value is within the desired range.
            if (newBone >= 0 && newBone <= 3)
            {
                bone = newBone;
                std::cout << "'bone' value updated to: " << bone << std::endl;
            }
            else
            {
                std::cout << "Invalid value. 'bone' value must be between 0 and 3." << std::endl;
            }
            
            // Clear the input buffer to prevent any issues with future input.
            std::cin.clear();
            std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        }
		if (userInput == "9")
        {
            //Loot Filled.
            lootfilledtoggle = !lootfilledtoggle;
            
            if (lootfilledtoggle)
            {
				lootfilled = 14;
                std::cout << "Loot Glow Filled.\n";
            }
            else
            {
				lootfilled = 0;
                std::cout << "Loot Glow Not Filled.\n";
            }
        }
		if (userInput == "10")
        {
            //player Filled.
            playerfilledtoggle = !playerfilledtoggle;
            
            if (playerfilledtoggle)
            {
				insidevalue = 14;
                std::cout << "Player Glow Filled.\n";
            }
            else
            {
				insidevalue = 0;
                std::cout << "Player Glow Not Filled.\n";
            }
        }
		if (userInput == "11")
        {
            // Command to change the 'smooth' value.
            std::cout << "Enter a new value for Player Outlines (0 to 255): ";
            int newoutlinesize;
            std::cin >> newoutlinesize;

            // Check if the new value is within the desired range.
            if (newoutlinesize >= 0 && newoutlinesize <= 255)
            {
                outlinesize = newoutlinesize;
                std::cout << "Player Outline updated to: " << outlinesize << std::endl;
            }
            else
            {
                std::cout << "Invalid value. 'outlinesize' value must be between 0 and 255." << std::endl;
            }
            
            // Clear the input buffer to prevent any issues with future input.
            std::cin.clear();
            std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        }
		if (userInput == "12")
        {
            // Select a glow set (1 for "Not Visible," 2 for "Visible," 3 for "Knocked").
            std::cout << "Select Glow: 1 - Not Visible, 2 - Visible, 3 - Knocked (can do fractions IE: 0.863: ";
            std::cin >> userInput;
            int selectedSet = std::stoi(userInput);
            
            switch (selectedSet)
            {
                case 1:
                    updateGlowColor(glowrnot, glowgnot, glowbnot, "'Not Visible'");
                    break;
                case 2:
                    updateGlowColor(glowrviz, glowgviz, glowbviz, "'Visible'");
                    break;
                case 3:
                    updateGlowColor(glowrknocked, glowgknocked, glowbknocked, "'Knocked'");
                    break;
                default:
                    std::cout << "Invalid set selection. Please choose 1-3.\n";
                    break;
            }
        }
		
	}
	terminal_t = false;
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
	//Used to change things on a timer
	//std::thread updateInsideValue_thr;
	std::thread terminal_thr;
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
				updateInsideValue_t = false;
				terminal_t = false;
				item_t = false;
				g_Base = 0;

				aimbot_thr.~thread();
				actions_thr.~thread();
				cactions_thr.~thread();
				//Used to change things on a timer
				//updateInsideValue_thr.~thread();
				terminal_thr.~thread();
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
				//Used to change things on a timer
				//updateInsideValue_thr = std::thread(updateInsideValue);
				terminal_thr = std::thread(terminal);
				itemglow_thr = std::thread(item_glow_t);
				aimbot_thr.detach();
				actions_thr.detach();
				cactions_thr.detach();
				//Used to change things on a timer
				//updateInsideValue_thr.detach();
				terminal_thr.detach();
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