#! /bin/env python3

import time
import subprocess

from pypresence import Presence
from time import sleep

CLIENT_ID = 718109162923360327
CUSTOM_FORMATS = ["{status}", "{file}", "{duration}", "{position}", "{time_left}", "{artist}", 
                  "{album}", "{title}", "{date}", "{aaa_mode}", "{continue_}", "{play_library}"
                  "{play_sorted}", "{replaygain}", "{replaygain_limit}", "{replaygain_preamp}", "{repeat}", "{repeat_current}"
                  "{shuffle}", "{softvol}", "{vol_right}", "{vol_left}"]

def get_remote():
    try:
        return subprocess.check_output(['cmus-remote', '-Q'], text=True)
    except subprocess.CalledProcessError as e:
        if "cmus is not running" in e.output:
            return None

class Parser:
    def __init__(self):
        self.query = {}
        """
            status
            file
            duration
            position
            time_left
            artist
            album
            title
            date
            aaa_mode
            continue_
            play_library
            play_sorted
            replaygain
            replaygain_limit
            replaygain_preamp
            repeat
            repeat_current
            shuffle
            softvol
            vol_left
            vol_right
        """
        
    def parse(self, remote: str):
        for line in remote.split("\n"):
            if line.startswith("status"): self.query["status"] = line.replace("status ", "")
            if line.startswith("file"): self.query["file"] = line.replace("file ", "")
            if line.startswith("duration"): self.query["duration"] = line.replace("duration ", "")
            if line.startswith("position"): self.query["position"] = line.replace("position ", "")
            
            if self.query.get("duration") and self.query.get("position"): 
                timeleft = int(self.query["duration"]) - int(self.query["position"])
                self.query["time_left"] = time.strftime('%H:%M:%S', time.gmtime(timeleft))
            
            if line.startswith("tag artist"): self.query["artist"] = line.replace("tag artist ", "")
            if line.startswith("tag album"): self.query["album"] = line.replace("tag album ", "")
            if line.startswith("tag title"): self.query["title"] = line.replace("tag title ", "")
            if line.startswith("tag date"): self.query["date"] = line.replace("tag date ", "")
            if line.startswith("set aaa_mode"): self.query["aaa_mode"] = line.replace("set aaa_mode ", "")
            if line.startswith("set continue"):self.query["continue_"] = line.replace("set continue ", "")
            if line.startswith("set play_library"): self.query["play_library"] = line.replace("set play_library ", "")
            if line.startswith("set play_sorted"): self.query["play_sorted"] = line.replace("set play_sorted ", "")
            if line.startswith("set replaygain"): self.query["replaygain"] = line.replace("set replaygain ", "")
            if line.startswith("set replaygain_limit"): self.query["replaygain_limit"] = line.replace("set replaygain_limit ", "")
            if line.startswith("set replaygain_preamp"): self.query["replaygain_preamp"] = line.replace("set replaygain_preamp ", "")
            if line.startswith("set repeat"): self.query["repeat"] = line.replace("set repeat ", "")
            if line.startswith("set repeat_current"): self.query["repeat_current"] = line.replace("set repeat_current ", "")
            if line.startswith("set shuffle"): self.query["shuffle"] = line.replace("set shuffle ", "")
            if line.startswith("set softvol"): self.query["softvol"] = line.replace("set softvol ", "")
            if line.startswith("set vol_left"): self.query["vol_left"] = line.replace("set vol_left ", "")
            if line.startswith("set vol_right"): self.query["vol_right"] = line.replace("set vol_right ", "")

def format(string: str, query: dict):
    new_string = string
    
    for custom_format in CUSTOM_FORMATS:
        if custom_format in string:
            value: str = query.get(custom_format.lstrip("{").rstrip("}")) or ""
            new_string = new_string.replace(custom_format, value)
    
    return new_string
            

def main():
    top_text = "{title}"
    bottom_text = "{artist} ({time_left})"
    start = int(time.time())
    rpc = Presence(CLIENT_ID)
    rpc.connect()
    
    while True:
        remote = get_remote()
        parser = Parser()
        query = parser.query

        if remote:
            parser.parse(remote)
            if query.get("status") == "stopped" or query.get("status") == "paused": continue
            top = format(top_text, query)
            bottom = format(bottom_text, query)
            
            rpc.update(state = bottom, details=top, large_image="icon",
                       start=start)
                    
        sleep(1)
        
    rpc.close()
        
if __name__ == '__main__':
    main()