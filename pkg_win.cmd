@echo off
scp whatismyip.json maciakl@maciak.org:~/dmp.maciak.org/
cd target\release\
rm whatismyip_win.zip
7z a whatismyip_win.zip whatismyip.exe
