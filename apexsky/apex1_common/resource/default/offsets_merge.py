# 功能：
#   本脚本用于合并并规范 offsets.ini 中的偏移配置。
#   它会去重字段（并验证值是否一致）按偏移值排序，
#   并根据继承关系更新 DataMap 结构。
#
# Purpose:
#   This script merges and normalizes offset definitions from offsets.ini.
#   It deduplicates fields (ensuring consistency), sorts entries by offset value,
#   and updates DataMap sections based on inheritance hierarchy.
#
# Author: 🦅 | Co-Author: ChatGPT
# Date: April 7, 2025

import os
import re
from collections import defaultdict

inheritance_dict = {
    "DataMap.C_BaseEntity": ["RecvTable.DT_BaseEntity"],
    "DataMap.C_BaseCombatCharacter": [
        "RecvTable.DT_BaseEntity",
        "RecvTable.DT_BaseCombatCharacter",
    ],
    "DataMap.C_Player": [
        "RecvTable.DT_BaseEntity",
        "RecvTable.DT_BaseCombatCharacter",
        "RecvTable.DT_Player",
        #"RecvTable.DT_LocalPlayerExclusive",
    ],
}


def is_offset_section(section):
    return section.startswith("RecvTable.DT_") or section.startswith("DataMap.C_")


def get_offset_value(val):
    try:
        return int(val, 16)
    except:
        return 0


def normalize_key(key):
    return key.replace('"', "").strip().lower()


def parse_raw_ini_with_comments(file_path):
    with open(file_path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    sections = defaultdict(list)
    section_order = []
    current_section = None

    for line in lines:
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            current_section = stripped[1:-1]
            if current_section not in section_order:
                section_order.append(current_section)
            sections[current_section].append(line)  # 保留原始行
        elif current_section is not None:
            sections[current_section].append(line)
        else:
            sections[None].append(line)

    for section, lines in sections.items():
        if lines[-1] == "\n":
            lines.pop()

    return sections, section_order


def merge_and_process_sections(sections):
    processed = {}
    for section, lines in sections.items():
        if section is None:
            processed[section] = lines
            continue

        if not is_offset_section(section):
            # 非偏移结构，原样保留
            processed[section] = lines
            continue

        seen_keys = {}
        comments = []
        print(f"\n[合并排序 Section: {section}]")

        for line in lines:
            stripped = line.strip()
            if (
                not stripped
                or stripped.startswith(";")
                or stripped.startswith("#")
                or (stripped.startswith("[") and stripped.endswith("]"))
            ):
                comments.append(line)
                continue
            if "=" not in line:
                comments.append(line)
                continue

            key, val = map(str.strip, line.split("=", 1))
            norm_key = normalize_key(key)
            if norm_key in seen_keys:
                if seen_keys[norm_key][1] == val:
                    print(f"  ⚪ 字段重复，值相同：{key} = {val}（保留）")
                else:
                    print(
                        f'  ⚠️ 字段冲突：{key} -> "{seen_keys[norm_key][1]}" vs "{val}"（保留第一个）'
                    )
            else:
                seen_keys[norm_key] = (key, val)

        # 排序字段
        sorted_items = sorted(seen_keys.values(), key=lambda x: get_offset_value(x[1]))
        processed[section] = comments + [f"{k} = {v}\n" for k, v in sorted_items]

    return processed


def compare_and_sync_sections(processed, dest_sec, src_sec_mapping):
    if dest_sec not in processed:
        return

    # 目标条目获取
    dest_entries = [
        (normalize_key(line.split("=")[0]), line.strip())
        for line in processed[dest_sec]
        if "=" in line
    ]
    dest_map = {k: v for k, v in dest_entries}

    print(f"\n==== {dest_sec} 更新 ====")

    # 处理每个来源
    for src_sec, src_entries in src_sec_mapping.items():
        src_map = {
            normalize_key(line.split("=")[0]): line.strip() for line in src_entries
        }

        print(f"\n来自: {src_sec}")

        # 遍历源条目并对比更新
        for key in src_map:
            if key in dest_map:
                prev_val = dest_map[key].split("=")[1].strip()
                updated_val = src_map[key].split("=")[1].strip()

                if src_map[key] != dest_map[key]:
                    print(f"💡 {key}: {prev_val} -> {updated_val}")

                    # 添加注释，标明多个来源
                    comment = f"; {prev_val} -> {updated_val} from: {src_sec}\n"  # 注释行，说明更新来源
                    dest_map[key] = f"{comment}{src_map[key]}"  # 更新目标字段，加入注释
                else:
                    print(f"✅ {key}: {prev_val}")

                    # 添加注释，标明多个来源
                    comment = f"; from: {src_sec}\n"  # 注释行，说明更新来源
                    dest_map[key] = f"{comment}{src_map[key]}"  # 更新目标字段，加入注释

            else:
                # 新的字段直接加入，注释更新来源
                # print(f"添加新的字段 {key}: {src_map[key].split('=')[1].strip()}")
                # comment = f"; updated from: {src_sec}\n"  # 注释行，说明更新来源
                # dest_map[key] = f"{comment}{src_map[key]}"
                continue

    # 检查是否更新和多个来源的冲突
    for key in dest_map:
        existing_sources = [
            src
            for src in src_sec_mapping
            if key
            in {normalize_key(line.split("=")[0]) for line in src_sec_mapping[src]}
        ]
        if len(existing_sources) > 1:
            # 显示冲突来源
            print(f"⚠️ 冲突：字段 {key} 更新来自多个来源: {', '.join(existing_sources)}")
        elif len(existing_sources) == 0:
            # 提示未更新
            print(f"⚠️ 字段 {key} 未更新")
            comment = f"; old\n"  # 注释行，说明未更新
            dest_map[key] = f"{comment}{dest_map[key]}"

    # 生成更新后的 dest_section 内容
    updated_lines = list(dest_map.values())
    updated_lines.sort(key=lambda x: get_offset_value(x.split("=")[1].strip()))

    # 保留原始注释行
    comments = [
        line
        for line in processed[dest_sec]
        if "=" not in line or line.strip().startswith(("#", ";"))
    ]

    # 更新 processed 字典中的目标部分
    processed[dest_sec] = comments + [line + "\n" for line in updated_lines]


def write_processed_ini(processed, section_order, output_path):
    with open(output_path, "w", encoding="utf-8") as f:
        if None in processed:
            f.writelines(processed[None])

        for section in section_order:
            if section not in processed:
                continue

            f.write(f"\n[{section}]\n")
            for line in processed[section]:
                if line.strip() == f"[{section}]":
                    continue  # 跳过原始 section 标题，避免重复
                if not line.endswith("\n"):
                    line += "\n"
                f.write(line)


def main():
    # work_path = os.path.join(os.getcwd(), "apexsky/apex1_common/resource/default")
    work_path = os.getcwd()
    ini_path = os.path.join(work_path, "offsets.ini")
    output_path = os.path.join(work_path, "offsets_merged.ini")

    if not os.path.exists(ini_path):
        print("❌ 找不到 offsets.ini 文件")
        return

    # Step 1: 读取并保留注释结构
    sections, section_order = parse_raw_ini_with_comments(ini_path)

    # Step 2: 合并并排序偏移字段
    processed = merge_and_process_sections(sections)

    # Step 3: 比较并同步字段
    for dest_sec, src_secs in inheritance_dict.items():
        # 构建src_sec_mapping字典
        src_sec_mapping = {}
        for src_sec in src_secs:
            # 如果存在相应的条目，则从processed中提取相应的行并将其添加到src_sec_mapping
            if src_sec in processed:
                src_sec_mapping[src_sec] = [
                    line.strip() for line in processed[src_sec] if "=" in line
                ]

        # 调用compare_and_sync_sections，传递目标部分和源部分
        compare_and_sync_sections(processed, dest_sec, src_sec_mapping)

    # Step 4: 写入新文件
    write_processed_ini(processed, section_order, output_path)
    print(f"\n✅ 合并完成，输出文件: {output_path}")


if __name__ == "__main__":
    main()
