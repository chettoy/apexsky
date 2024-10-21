import requests
from bs4 import BeautifulSoup
import json

url = "https://apexlegendsstatus.com/live-ranked-leaderboards/Battle_Royale/PC"

headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36"
}

response = requests.get(url, headers=headers)

if response.status_code == 200:
    html_content = response.text
    
    soup = BeautifulSoup(html_content, 'html.parser')
    
    user_modules = soup.find_all('a', {'target': '_alsLeaderboard'})
    rank_modules = soup.find_all('b')

    result = []
    for user_module, rank_module in zip(user_modules[:300], rank_modules[:300]):
        rank_text = rank_module.get_text()
        if "Player missing" not in rank_text:
            link = user_module['href']
            user_id = link.split('/')[-1]
            username = user_module.get_text()
            rank = rank_text
            result.append({"name": username, "uid": user_id, "rank": rank})
            print(f"用户ID: {user_id}, 用户名: {username}, 排名: {rank}")

    result_str = json.dumps(result, ensure_ascii=False)
    print(result_str)
else:
    print("请求失败")
