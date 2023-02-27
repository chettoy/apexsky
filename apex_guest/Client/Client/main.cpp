#pragma warning (disable : 4715)
#pragma warning (disable : 4005)
#pragma warning (disable : 4305)
#pragma warning (disable : 4244)
#include "main.h"
#include <random>
#include <map>

typedef struct player
{
	float dist = 0;
	int entity_team = 0;
	float boxMiddle = 0;
	float h_y = 0;
	float width = 0;
	float height = 0;
	float b_x = 0;
	float b_y = 0;
	bool knocked = false;
	bool visible = false;
	int health = 0;
	int shield = 0;
	//seer
	int maxshield = 0;
	int armortype = 0;
	D3DXVECTOR3 EntityPosition;
	D3DXVECTOR3 LocalPlayerPosition;
	D3DXVECTOR3 localviewangle;
	float targetyaw = 0;
	char name[33] = { 0 };
}player;


uint32_t check = 0xABCD;

//Aiming keys: left and right mouse button
int aim_key = VK_LBUTTON; //Left Click
int aim_key2 = VK_RBUTTON; //Right Click
int shoot_key = VK_LBUTTON; //Left Click
int shoot_key2 = VK_RBUTTON; //Right Click
//Left and Right Aim key toggle
bool toggleaim = false;
bool toggleaim2 = false;
extern int e;
bool firing_range = false;
bool use_nvidia = true; //Nvidia Shadowplay Overlay
bool active = true;
bool ready = false;
extern visuals v;
int aim = 2; //read
bool esp = true; //read
bool item_glow = true;
bool player_glow = true;
bool aim_no_recoil = true;
bool aiming = false; //read
uint64_t g_Base = 0; //write
float max_dist = 3800.0f * 40.0f; //read //Max Distance of ESP 3800 is full map
float smooth = 110.0f; //Min 100 for safe aimbotting
float max_fov = 15.0f; //15 is the sweetspot for 1080p
// Dynamic Fov
float dynamicfov = 10;
float dynamicfovmax = 15.0f;

float smoothpred = 0.08;
float smoothpred2 = 0.05;
float veltest = 1.00;
int bone = 2; //0 Head, 1 Neck, 2 Body, 3 Stomace, 4 Nuts
//Player Glow Color and Brightness
float glowr = 120.0f; //Red Value
float glowg = 0.0f; //Green Value
float glowb = 0.0f; //Blue Value
float glowcolor[3] = { 000.0f, 000.0f, 000.0f };
//more glow stuff
//glow visable
float glowrviz = 50.0f;
float glowgviz = 50.0f;
float glowbviz = 50.0f;
float glowcolorviz[3] = { 000.0f, 000.0f, 000.0f };
//knocked
float glowrknocked = 50.0f;
float glowgknocked = 50.0f;
float glowbknocked = 50.0f;
float glowcolorknocked[3] = { 000.0f, 000.0f, 000.0f };
extern int minimapradardotsize1;
extern int minimapradardotsize2;
bool minimapradar = false;
extern unsigned int radarcolorr; //Red Value
extern unsigned int radarcolorg; //Green Value
extern unsigned int radarcolorb; //Blue Value
float radarcolor[3];
//Full Map Radar
bool mainradartoggle = 0; //Toggle for Main Map radar
bool mainradarmap = false; //if the Main Map Radar is enabled
bool kingscanyon = false; //Set for map, ONLY ONE THO
bool stormpoint = true; //Set for map, ONLY ONE THO
extern int mainmapradardotsize1;
extern int mainmapradardotsize2;
//New Radar test
bool mapradartest = false;
//Ha think i was done ?
//Item Filter Brute Force!
bool lightbackpack = false;
bool medbackpack = false;
bool heavybackpack = false;
bool shieldupgrade = false;
bool shieldupgradehead = false;
bool accelerant = false;
bool phoenix = false;
bool healthlarge = false;
bool healthsmall = false;
bool shieldbattsmall = false;
bool shieldbattlarge = false;
bool ammosniper = false;
bool ammohc = false;
bool optic = false;
bool ammosc = false;
bool ammonrg = false;
bool ammoshotgun = false;
bool lasersight = false;
bool magsniper = false;
bool magenergy = false;
bool stocksniper = false;
bool stockregular = false;
bool shielddown = false;
bool lightammomag = false;
bool heavyammomag = false;
bool optic2x = false;
bool opticholo1x = false;
bool opticsniper6x = false;
bool opticsniper4x8x = false;
bool opticsniperthreat = false;
bool optic2x4x = false;
bool opticthreat = false;
bool optic3x = false;
bool opticholo1x2x = false;
bool suppressor = false;
bool weaponmod = false;
bool grenade_frag = false;
bool grenade_arc_star = false;
bool grenade_thermite = false;
bool shotgunbolt = false;
bool weapon_kraber = false;
bool weapon_mastiff = false;
bool weapon_lstar = false;
bool weapon_havoc = false;
bool weapon_devotion = false;
bool weapon_triple_take = false;
bool weapon_flatline = false;
bool weapon_hemlock = false;
bool weapon_g7_scout = false;
bool weapon_alternator = false;
bool weapon_r99 = false;
bool weapon_prowler = false;
bool weapon_volt = false;
bool weapon_longbow = false;
bool weapon_charge_rifle = false;
bool weapon_spitfire = false;
bool weapon_r301 = false;
bool weapon_eva8 = false;
bool weapon_peacekeeper = false;
bool weapon_mozambique = false;
bool weapon_wingman = false;
bool weapon_p2020 = false;
bool weapon_re45 = false;
bool weapon_sentinel = false;
bool weapon_bow = false;
bool weapon_3030_repeater = false;
bool weapon_rampage = false;
bool weapon_car_smg = false;
bool weapon_nemesis = false;
bool weapon_rampage_lmg = false;
// Aim distance check
float aimdist = 9905.0f;
//item glow brightness
int itemglowbrightness = 10;
//headshot mode
int snipereq = 0;
int bowheadshotmode = 0;


bool thirdperson = false;
int spectators = 0; //write
int allied_spectators = 0; //write
bool valid = true; //write
bool next2 = true; //read write

uint64_t add[106];

bool k_f5 = 0;
bool k_f6 = 0;
bool k_f7 = 0;
bool k_f8 = 0;
bool k_f9 = 0;
bool k_f10 = 0;


bool k_f20 = 0;

bool k_f100 = 0;

player players[100];

//Radar Code
#define M_PI		3.14159265358979323846	// matches value in gcc v2 math.h ?
static D3DXVECTOR3 RotatePoint(D3DXVECTOR3 EntityPos, D3DXVECTOR3 LocalPlayerPos, int posX, int posY, int sizeX, int sizeY, float angle, float zoom, bool* viewCheck)
{
	float r_1, r_2;
	float x_1, y_1;
	r_1 = -(EntityPos.y - LocalPlayerPos.y);
	r_2 = EntityPos.x - LocalPlayerPos.x;
	float Yaw = angle - 90.0f;
	float yawToRadian = Yaw * (float)(M_PI / 180.0F);
	x_1 = (float)(r_2 * (float)cos((double)(yawToRadian)) - r_1 * sin((double)(yawToRadian))) / 20;
	y_1 = (float)(r_2 * (float)sin((double)(yawToRadian)) + r_1 * cos((double)(yawToRadian))) / 20;
	*viewCheck = y_1 < 0;
	x_1 *= zoom;
	y_1 *= zoom;
	int sizX = sizeX / 2;
	int sizY = sizeY / 2;
	x_1 += sizX;
	y_1 += sizY;
	if (x_1 < 5)
		x_1 = 5;
	if (x_1 > sizeX - 5)
		x_1 = sizeX - 5;
	if (y_1 < 5)
		y_1 = 5;
	if (y_1 > sizeY - 5)
		y_1 = sizeY - 5;
	x_1 += posX;
	y_1 += posY;
	return D3DXVECTOR3(x_1, y_1, 0);
}
struct RGBA {
	int R;
	int G;
	int B;
	int A;
};
std::map<int, RGBA> teamColors;
static void TeamMiniMap(int x, int y, int radius, int teamID)
{
	RGBA color;
	auto it = teamColors.find(teamID);
	if (it == teamColors.end()) {
		// Define the minimum sum of RGB values for a color to be considered "light"
		const int MIN_SUM_RGB = 500;

		// Generate a new random color for this team, discarding colors with a low sum of RGB values
		std::random_device rd;
		std::mt19937 gen(rd());
		std::uniform_int_distribution<> dis(0, 255);
		RGBA color;
		do {
			color = { dis(gen), dis(gen), dis(gen), 255 };
		} while (color.R + color.G + color.B < MIN_SUM_RGB);

		// Store the color in the teamColors map
		teamColors[teamID] = color;
	}
	else {
		// Use the previously generated color for this team
		color = it->second;
	}
	
	auto colOutline = ImGui::ColorConvertFloat4ToU32(ImVec4(0.0, 0.0, 0.0, 1.0));
	ImGui::GetWindowDrawList()->AddCircleFilled(ImVec2(x, y), radius, ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0, color.B / 255.0, color.A / 255.0)));
	ImGui::GetWindowDrawList()->AddCircle(ImVec2(x, y), radius, colOutline, 12, minimapradardotsize2);
}
bool menu = true;
bool firstS = true;
namespace RadarSettings
{
	bool Radar = true;
	bool teamRadar = true;
	bool enemyRadar = true;
	int xAxis_Radar = 0;
	int yAxis_Radar = 400;
	int radartype = 0;
	int width_Radar = 400;
	int height_Radar = 400;
	int distance_Radar = 250;
	int distance_Radar2 = 1000;
};
void DrawRadarPoint(D3DXVECTOR3 EneamyPos, D3DXVECTOR3 LocalPos, float LocalPlayerY, float eneamyDist, int TeamID, int xAxis, int yAxis, int width, int height, D3DXCOLOR color, float targetyaw)
{
	bool out = false;
	D3DXVECTOR3 siz;
	siz.x = width;
	siz.y = height;
	D3DXVECTOR3 pos;
	pos.x = xAxis;
	pos.y = yAxis;
	bool ck = false;
	D3DXVECTOR3 single = RotatePoint(EneamyPos, LocalPos, pos.x, pos.y, siz.x, siz.y, LocalPlayerY, 0.3f, &ck);
	if (eneamyDist >= 0.f && eneamyDist < RadarSettings::distance_Radar)
	{
		for (int i = 1; i <= 30; i++)
		{
			TeamMiniMap(single.x, single.y, minimapradardotsize1, TeamID);
		}
	}
}
void MiniMapRadar(D3DXVECTOR3 EneamyPos, D3DXVECTOR3 LocalPos, float LocalPlayerY, float eneamyDist, int TeamId, float targetyaw)
{
	ImGuiStyle* style = &ImGui::GetStyle();
	style->WindowRounding = 0.2f;
	ImGui::PushStyleColor(ImGuiCol_WindowBg, ImVec4(0.13529413f, 0.14705884f, 0.15490198f, 0.82f));
	ImGuiWindowFlags TargetFlags;
	TargetFlags = ImGuiWindowFlags_::ImGuiWindowFlags_NoResize | ImGuiWindowFlags_::ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_::ImGuiWindowFlags_NoBackground | ImGuiWindowFlags_::ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_::ImGuiWindowFlags_NoMove;
	if (!firstS)
	{
		ImGui::SetNextWindowPos(ImVec2{ 1200, 60 }, ImGuiCond_Once);
		firstS = true;
	}
	if (RadarSettings::Radar == true)
	{
		ImGui::SetNextWindowSize({ 250, 250 });
		ImGui::Begin(("Radar"), 0, TargetFlags);
		{
			ImDrawList* Draw = ImGui::GetWindowDrawList();
			ImVec2 DrawPos = ImGui::GetCursorScreenPos();
			ImVec2 DrawSize = ImGui::GetContentRegionAvail();
			ImVec2 midRadar = ImVec2(DrawPos.x + (DrawSize.x / 2), DrawPos.y + (DrawSize.y / 2));
			DrawRadarPoint(EneamyPos, LocalPos, LocalPlayerY, eneamyDist, TeamId, DrawPos.x, DrawPos.y, DrawSize.x, DrawSize.y, { 255, 255, 255, 255 }, targetyaw);
		}
		ImGui::End();
	}
	ImGui::PopStyleColor();
}
bool IsKeyDown(int vk)
{
	return (GetAsyncKeyState(vk) & 0x8000) != 0;
}
void Overlay::RenderEsp()
{
	next2 = false;
	if (g_Base != 0 && esp)
	{
		memset(players, 0, sizeof(players));	
		while (!next2 && esp)
		{
			std::this_thread::sleep_for(std::chrono::milliseconds(1));
		}
		if (next2 && valid)
		{
			ImGui::SetNextWindowPos(ImVec2(0, 0));
			ImGui::SetNextWindowSize(ImVec2((float)getWidth(), (float)getHeight()));
			ImGui::Begin(XorStr("##esp"), (bool*)true, ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoBackground | ImGuiWindowFlags_NoBringToFrontOnFocus);
			for (int i = 0; i < 100; i++)
			{
				if (players[i].health > 0)
				{
					std::string distance = std::to_string(players[i].dist / 39.62);
					distance = distance.substr(0, distance.find('.')) + "m(" + std::to_string(players[i].entity_team) + ")";
					float radardistance = (int)((players[i].LocalPlayerPosition, players[i].dist) / 39.62);
					if (minimapradar == true)
					{
							MiniMapRadar(players[i].EntityPosition, players[i].LocalPlayerPosition, players[i].localviewangle.y, radardistance, players[i].entity_team, players[i].targetyaw);
					}
					if (v.line)
						DrawLine(ImVec2((float)(getWidth() / 2), (float)getHeight()), ImVec2(players[i].b_x, players[i].b_y), BLUE, 1); //LINE FROM MIDDLE SCREEN
					if (v.distance)
					{
						if (players[i].knocked)
							String(ImVec2(players[i].boxMiddle, (players[i].b_y + 1)), RED, distance.c_str());  //DISTANCEs			else
							String(ImVec2(players[i].boxMiddle, (players[i].b_y + 1)), GREEN, distance.c_str());  //DISTANCE
					}
					if (v.healthbar)
					{
						if (players[i].dist < 16000.0f)
						{
							DrawSeerLikeHealth((players[i].b_x - (players[i].width / 2.0f) + 5), (players[i].b_y - players[i].height - 10), players[i].shield, players[i].maxshield, players[i].armortype, players[i].health); //health bar
						}
					}
				}
			}
			ImGui::End();
		}
	}
}

int main(int argc, char** argv)
{
	add[0] = (uintptr_t)&check;
	add[1] = (uintptr_t)&aim;
	add[2] = (uintptr_t)&esp;
	add[3] = (uintptr_t)&aiming;
	add[4] = (uintptr_t)&g_Base;
	add[5] = (uintptr_t)&next2;
	add[6] = (uintptr_t)&players[0];
	add[7] = (uintptr_t)&valid;
	add[8] = (uintptr_t)&max_dist;
	add[9] = (uintptr_t)&item_glow;
	add[10] = (uintptr_t)&player_glow;
	add[11] = (uintptr_t)&aim_no_recoil;
	add[12] = (uintptr_t)&smooth;
	add[13] = (uintptr_t)&max_fov;
	add[14] = (uintptr_t)&bone;
	add[15] = (uintptr_t)&thirdperson;
	add[16] = (uintptr_t)&spectators;
	add[17] = (uintptr_t)&allied_spectators;
	add[18] = (uintptr_t)&glowr;
	add[19] = (uintptr_t)&glowg;
	add[20] = (uintptr_t)&glowb;
	add[21] = (uintptr_t)&firing_range;
	add[22] = (uintptr_t)&lightbackpack;
	add[23] = (uintptr_t)&medbackpack;
	add[24] = (uintptr_t)&heavybackpack;
	add[25] = (uintptr_t)&shieldupgrade;
	add[26] = (uintptr_t)&shieldupgradehead;
	add[27] = (uintptr_t)&accelerant;
	add[28] = (uintptr_t)&phoenix;
	add[29] = (uintptr_t)&healthlarge;
	add[30] = (uintptr_t)&healthsmall;
	add[31] = (uintptr_t)&shieldbattsmall;
	add[32] = (uintptr_t)&shieldbattlarge;
	add[33] = (uintptr_t)&ammosniper;
	add[34] = (uintptr_t)&ammohc;
	add[35] = (uintptr_t)&optic;
	add[36] = (uintptr_t)&ammosc;
	add[37] = (uintptr_t)&ammonrg;
	add[38] = (uintptr_t)&ammoshotgun;
	add[39] = (uintptr_t)&lasersight;
	add[40] = (uintptr_t)&magsniper;
	add[41] = (uintptr_t)&magenergy;
	add[42] = (uintptr_t)&stocksniper;
	add[43] = (uintptr_t)&stockregular;
	add[44] = (uintptr_t)&shielddown;
	add[45] = (uintptr_t)&lightammomag;
	add[46] = (uintptr_t)&heavyammomag;
	add[47] = (uintptr_t)&optic2x;
	add[48] = (uintptr_t)&opticholo1x;
	add[49] = (uintptr_t)&opticholo1x2x;
	add[50] = (uintptr_t)&opticthreat;
	add[51] = (uintptr_t)&optic3x;
	add[52] = (uintptr_t)&optic2x4x;
	add[53] = (uintptr_t)&opticsniper6x;
	add[54] = (uintptr_t)&opticsniper4x8x;
	add[55] = (uintptr_t)&opticsniperthreat;
	add[56] = (uintptr_t)&suppressor;
	add[57] = (uintptr_t)&weaponmod;
	add[58] = (uintptr_t)&grenade_frag;
	add[59] = (uintptr_t)&grenade_arc_star;
	add[60] = (uintptr_t)&grenade_thermite;
	add[61] = (uintptr_t)&shotgunbolt;
	add[62] = (uintptr_t)&weapon_kraber;
	add[63] = (uintptr_t)&weapon_mastiff;
	add[64] = (uintptr_t)&weapon_lstar;
	add[65] = (uintptr_t)&weapon_havoc;
	add[66] = (uintptr_t)&weapon_devotion;
	add[67] = (uintptr_t)&weapon_triple_take;
	add[68] = (uintptr_t)&weapon_flatline;
	add[69] = (uintptr_t)&weapon_hemlock;
	add[70] = (uintptr_t)&weapon_g7_scout;
	add[71] = (uintptr_t)&weapon_alternator;
	add[72] = (uintptr_t)&weapon_r99;
	add[73] = (uintptr_t)&weapon_prowler;
	add[74] = (uintptr_t)&weapon_volt;
	add[75] = (uintptr_t)&weapon_longbow;
	add[76] = (uintptr_t)&weapon_charge_rifle;
	add[77] = (uintptr_t)&weapon_spitfire;
	add[78] = (uintptr_t)&weapon_r301;
	add[79] = (uintptr_t)&weapon_eva8;
	add[80] = (uintptr_t)&weapon_peacekeeper;
	add[81] = (uintptr_t)&weapon_mozambique;
	add[82] = (uintptr_t)&weapon_wingman;
	add[83] = (uintptr_t)&weapon_p2020;
	add[84] = (uintptr_t)&weapon_re45;
	add[85] = (uintptr_t)&weapon_sentinel;
	add[86] = (uintptr_t)&weapon_bow;
	add[87] = (uintptr_t)&weapon_3030_repeater;
	add[88] = (uintptr_t)&weapon_rampage;
	add[89] = (uintptr_t)&weapon_car_smg;
	add[90] = (uintptr_t)&aimdist;
	add[91] = (uintptr_t)&itemglowbrightness;
	//glow visable
	add[92] = (uintptr_t)&glowrviz;
	add[93] = (uintptr_t)&glowgviz;
	add[94] = (uintptr_t)&glowbviz;
	//knocked
	add[95] = (uintptr_t)&glowrknocked;
	add[96] = (uintptr_t)&glowgknocked;
	add[97] = (uintptr_t)&glowbknocked;
	add[98] = (uintptr_t)&smoothpred;
	add[99] = (uintptr_t)&smoothpred2;
	add[100] = (uintptr_t)&weapon_nemesis;
	add[101] = (uintptr_t)&mapradartest;
	add[102] = (uintptr_t)&weapon_rampage_lmg;
	add[103] = (uintptr_t)&snipereq;
	add[104] = (uintptr_t)&bowheadshotmode;
	add[105] = (uintptr_t)&veltest;
	
	

	
	printf(XorStr("GameVersion v3.0.26.26 || 2-26-2023 || |-| Clean up with ChatGPT |-| Add me offset: 0x%I64x\n"), (uint64_t)&add[0] - (uint64_t)GetModuleHandle(NULL));

	Overlay ov1 = Overlay();
	ov1.Start();
	printf(XorStr("Waiting for The Ban Hammer .... Never Gonna Get it!\n"));
	while (check == 0xABCD)
	{
		if (IsKeyDown(VK_F4))
		{
			active = false;
			break;
		}
		std::this_thread::sleep_for(std::chrono::milliseconds(1));
	}
	if (active)
	{
		ready = true;
		printf(XorStr("Ready To Bring The Pain\n"));

	}

	while (active)
	{
		std::this_thread::sleep_for(std::chrono::milliseconds(1));
		if (IsKeyDown(VK_F4))
		{
			active = false;
		}

		//Load at start for saved settings to take effect. Need to save once to make the file. 

		for (static bool once = true; once; once = false) {
			std::ifstream config("Settings.txt");
			if (config.is_open())
			{
				config >> std::boolalpha >> firing_range;
				config >> aim;
				config >> std::boolalpha >> esp;
				config >> std::boolalpha >> item_glow;
				config >> std::boolalpha >> player_glow;
				config >> std::boolalpha >> aim_no_recoil;
				config >> max_dist;
				config >> smooth;
				config >> max_fov;
				config >> bone;
				config >> glowr;
				config >> glowg;
				config >> glowb;
				config >> glowcolor[0];
				config >> glowcolor[1];
				config >> glowcolor[2];
				config >> radarcolorr;
				config >> radarcolorg;
				config >> radarcolorb;
				config >> radarcolor[0];
				config >> radarcolor[1];
				config >> radarcolor[2];
				config >> v.healthbar;
				config >> v.shieldbar;
				config >> v.distance;
				config >> thirdperson;
				config >> minimapradar;
				config >> lightbackpack;
				config >> medbackpack;
				config >> heavybackpack;
				config >> shieldupgrade;
				config >> shieldupgradehead;
				config >> accelerant;
				config >> phoenix;
				config >> healthlarge;
				config >> healthsmall;
				config >> shieldbattsmall;
				config >> shieldbattlarge;
				config >> ammosniper;
				config >> ammohc;
				config >> optic;
				config >> ammosc;;
				config >> ammonrg;
				config >> ammoshotgun;
				config >> lasersight;;
				config >> magsniper;
				config >> magenergy;
				config >> stocksniper;
				config >> stockregular;
				config >> shielddown;
				config >> lightammomag;
				config >> heavyammomag;
				config >> optic2x;
				config >> opticholo1x;
				config >> opticsniper6x;
				config >> opticsniper4x8x;
				config >> opticsniperthreat;
				config >> optic2x4x;
				config >> opticthreat;
				config >> optic3x;
				config >> opticholo1x2x;

				config >> suppressor;
				config >> weaponmod;
				config >> grenade_frag;
				config >> grenade_arc_star;
				config >> grenade_thermite;
				config >> shotgunbolt;
				config >> weapon_kraber;
				config >> weapon_mastiff;
				config >> weapon_lstar;
				config >> weapon_havoc;
				config >> weapon_devotion;
				config >> weapon_triple_take;
				config >> weapon_flatline;
				config >> weapon_hemlock;
				config >> weapon_g7_scout;
				config >> weapon_alternator;
				config >> weapon_r99;
				config >> weapon_prowler;
				config >> weapon_volt;
				config >> weapon_longbow;
				config >> weapon_charge_rifle;
				config >> weapon_spitfire;
				config >> weapon_r301;
				config >> weapon_eva8;
				config >> weapon_peacekeeper;
				config >> weapon_mozambique;
				config >> weapon_wingman;
				config >> weapon_p2020;
				config >> weapon_re45;
				config >> weapon_sentinel;
				config >> weapon_bow;
				config >> weapon_3030_repeater;
				config >> weapon_rampage;
				config >> weapon_car_smg;
				config >> toggleaim;
				config >> toggleaim2;
				config >> e;
				config >> minimapradardotsize1;
				config >> minimapradardotsize2;
				config >> aimdist;
				config >> itemglowbrightness;
				config >> mainmapradardotsize1;
				config >> mainmapradardotsize2;
				config >> dynamicfov;
				config >> dynamicfovmax;

				//glow visable
				config >> glowrviz;
				config >> glowgviz;
				config >> glowbviz;
				config >> glowcolorviz[0];
				config >> glowcolorviz[1];
				config >> glowcolorviz[2];
				//glow knocked
				config >> glowrknocked;
				config >> glowgknocked;
				config >> glowbknocked;
				config >> glowcolorknocked[0];
				config >> glowcolorknocked[1];
				config >> glowcolorknocked[2];
				config >> smoothpred;
				config >> smoothpred2;
				config >> weapon_nemesis;
				config >> weapon_rampage_lmg;
				config >> veltest;
				config.close();
			}
		}
		//Hotkey to Turn on and off Aimbot
		if (IsKeyDown(VK_F6) && k_f6 == 0)
		{
			k_f6 = 1;
			switch (aim)
			{
			case 0:
				aim = 1;
				break;
			case 1:
				aim = 2;
				break;
			case 2:
				aim = 0;
				break;
			default:
				break;
			}
		}
		else if (!IsKeyDown(VK_F6) && k_f6 == 1)
		{
			k_f6 = 0;
		}
		//Main Map Radar, Needs Manual Setting of cords
		/*if (IsKeyDown(0x4D) && mainradartoggle == 0)
		{
			mainradartoggle = 1;
			switch (mainradarmap)
			{
			case 0:
				mainradarmap = true;
				minimapradar = false;
				break;
			case 1:
				mainradarmap = false;
				minimapradar = true;
				break;
			}
		}
		else if (!IsKeyDown(0x4D) && mainradartoggle == 1)
		{
			mainradartoggle = 0;
		}
		*/
		//New Radar test

		if (IsKeyDown(VK_END))
		{
			
			mapradartest = true;
			Sleep(300);
			mapradartest = false;
			

		}


		
		if (IsKeyDown(aim_key) && toggleaim)
		{
			aiming = true;
		}

		else if (IsKeyDown(aim_key2) && toggleaim2)
			aiming = true;
		else
		{
			aiming = false;
		}
		
	}
	ready = false;
	ov1.Clear();
	if (!use_nvidia)
		system(XorStr("taskkill /F /T /IM overlay_ap.exe")); //custom overlay process name
	return 0;
}



