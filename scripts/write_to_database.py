import os
import json
import sqlite3
import parse_songs

songs_dir = f"{os.path.dirname(__file__)}/songs"

file_content = open(f"{songs_dir}/english.json").read()
song_json = json.loads(file_content)
songs = parse_songs.parse_songs(song_json)

dir_parts = os.path.dirname(__file__).split('/')[:-1]
dir_parts.append('database.db')
db_dir = "/".join(dir_parts)

con = sqlite3.connect(db_dir)
cur = con.cursor()

# for song in songs:
#     # Check if song exists, otherwise create the song
#     result = cur.execute("SELECT * FROM song WHERE id = ?", (song["number"],))
#     if result.rowcount <= 0: #Does not exist
#         cur.execute("INSERT INTO song (book, sequence, number) VALUES (?, ?, ?)", (1, song["sequence"], song["number"]))
#         con.commit()
#     for section in song["content"]:
#         cur.execute("INSERT INTO song_section (song, section, language, content) VALUES (?, ?, ?, ?)", (song["number"], section["section"], 'EN', section["lyrics"]))
#         con.commit()

for song in songs:
    cur.execute("INSERT INTO song_title (song, language, title) VALUES (?, ?, ?)", (song["number"], "EN", song["title"]))
    con.commit()