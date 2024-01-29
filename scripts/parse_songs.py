import re

def parse_songs(song_json):
    songs = []
    for song in song_json:
        title = " ".join(song['title'].split(" ")[1:])
        title = re.sub(r'[“|”|.]', '', title).strip()
        number = int(song["number"])
        markdown: str = song['markdown']
        parts_markdown = markdown.split('\n\n')[1:]
        verse = 1
        song_parsed = {
            "title": title,
            "number": number,
            "content": [],
            "sequence": ""
        }
        for part in parts_markdown:
            raw_lines = part.split('\n')
            is_chorus = False
            lines = []
            for line in raw_lines:
                line = line.strip()
                if line == '': continue
                if line == "**CHORUS:**": 
                    is_chorus = True
                    verse = verse - 1
                else: lines.append(line)
            content = '\n'.join(lines)
            section = "C" if is_chorus else f"V{verse}"
            song_parsed["content"].append({"section": section, "lyrics": content})
            verse = verse + 1
        sections_old = list(map(lambda section: section["section"], song_parsed["content"]))
        sections = []
        if 'C' in sections_old:
            for section in sections_old:
                if section == 'C': continue
                sections.append(section)
                sections.append('C')
        else:
            sections = sections_old

        sequence = "-".join(sections)
        song_parsed["sequence"] = sequence

        songs.append(song_parsed)
    return songs